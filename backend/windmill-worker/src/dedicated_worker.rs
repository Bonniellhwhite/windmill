// use tokio::sync::mpsc;

// pub fn create_dedicated_worker() {
//     let (job_completed_tx, mut new_job) = mpsc::channel::<JobCompleted>(100);
// }

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
};
use windmill_common::{error, jobs::QueuedJob, variables};

use std::{collections::VecDeque, process::Stdio, sync::Arc};

use anyhow::Context;

use crate::{common::start_child_process, JobCompleted, MAX_BUFFERED_DEDICATED_JOBS};

use futures::{future, Future};
use std::{collections::HashMap, task::Poll};

use tokio::sync::mpsc::{Receiver, Sender};

fn conditional_polling<T>(
    fut: impl Future<Output = T>,
    predicate: bool,
) -> impl Future<Output = T> {
    let mut fut = Box::pin(fut);
    future::poll_fn(move |cx| {
        if predicate {
            fut.as_mut().poll(cx)
        } else {
            Poll::Pending
        }
    })
}

async fn write_stdin(stdin: &mut tokio::process::ChildStdin, s: &str) -> error::Result<()> {
    let _ = &stdin.write_all(format!("{s}\n").as_bytes()).await?;
    stdin.flush().await.context("stdin flush")?;
    Ok(())
}

pub async fn handle_dedicated_process(
    command_path: &String,
    job_dir: &str,
    context_envs: HashMap<String, String>,
    envs: HashMap<String, String>,
    reserved_variables: [variables::ContextualVariable; 15],
    common_bun_proc_envs: HashMap<String, String>,
    args: Vec<&str>,
    mut killpill_rx: tokio::sync::broadcast::Receiver<()>,
    job_completed_tx: Sender<JobCompleted>,
    token: &str,
    mut jobs_rx: Receiver<Arc<QueuedJob>>,
) -> std::result::Result<(), error::Error> {
    //do not cache local dependencies
    let mut child = {
        let mut cmd = Command::new(command_path);
        cmd.current_dir(job_dir)
            .env_clear()
            .envs(context_envs)
            .envs(envs)
            .envs(
                reserved_variables
                    .iter()
                    .map(|x| (x.name.clone(), x.value.clone()))
                    .collect::<Vec<_>>(),
            )
            .envs(common_bun_proc_envs)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        start_child_process(cmd, command_path).await?
    };

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stdin");

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    let child = tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    let mut jobs = VecDeque::with_capacity(MAX_BUFFERED_DEDICATED_JOBS);
    // let mut i = 0;
    // let mut j = 0;
    let mut alive = true;

    loop {
        tokio::select! {
            biased;
            _ = killpill_rx.recv(), if alive => {
                println!("received killpill for dedicated worker");
                alive = false;
                if let Err(e) = write_stdin(&mut stdin, "end").await {
                    tracing::info!("Could not write end message to stdin: {e:?}")
                }
            },
            line = reader.next_line() => {
                // j += 1;

                if let Some(line) = line.expect("line is ok") {
                    if line == "start" {
                        tracing::info!("dedicated worker process started");
                        continue;
                    }
                    tracing::debug!("processed job");

                    let result = serde_json::from_str(&line).expect("json is ok");
                    let job: Arc<QueuedJob> = jobs.pop_front().expect("pop");
                    job_completed_tx.send(JobCompleted { job , result, logs: "".to_string(), mem_peak: 0, success: true, cached_res_path: None, token: token.to_string() }).await.unwrap();
                } else {
                    tracing::info!("dedicated worker process exited");
                    break;
                }
            },
            job = conditional_polling(jobs_rx.recv(), alive && jobs.len() < MAX_BUFFERED_DEDICATED_JOBS) => {
                // i += 1;
                if let Some(job) = job {
                    tracing::debug!("received job");
                    jobs.push_back(job.clone());
                    // write_stdin(&mut stdin, &serde_json::to_string(&job.args.unwrap_or_else(|| serde_json::json!({"x": job.id}))).expect("serialize")).await?;
                    write_stdin(&mut stdin, &serde_json::to_string(&job.args).expect("serialize")).await?;
                    stdin.flush().await.context("stdin flush")?;
                } else {
                    tracing::debug!("job channel closed");
                    alive = false;
                    if let Err(e) = write_stdin(&mut stdin, "end").await {
                        tracing::error!("Could not write end message to stdin: {e:?}")
                    }
                }
            }
        }
    }

    child
        .await
        .map_err(|e| anyhow::anyhow!("child process encountered an error: {e}"))?;
    tracing::info!("dedicated worker child process exited successfully");
    Ok(())
}
