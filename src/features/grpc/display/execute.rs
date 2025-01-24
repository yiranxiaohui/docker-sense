use crate::features::docker::execute::{run_command, start_execute};
use crate::features::grpc::gen::docker::{ExecReply, ExecRequest};
use crate::features::grpc::status::Status;
use serde::Deserialize;
use serde_json::json;
use crate::features::docker::execute;

#[derive(Deserialize)]
struct Execute {
    #[serde(rename = "instanceId", )]
    instance_id: String,
    #[serde(rename = "executeId")]
    execute_id: String,
    command: String
}

pub async fn display_create_execute(data: ExecReply) {
    let container_id: String = data.status;
    let execute_id = match execute::create_execute(container_id).await {
        Ok(res) => {
            res.id
        }
        Err(_) => {
            String::new()
        }
    };
    let req = ExecRequest::ok(data.id, "/execute/create", execute_id).await.unwrap();
    Status::send(req).await;
}

pub async fn display_start_execute(data: ExecReply) {
    let execute_id: String = data.status;
    start_execute(execute_id).await;
    let req = ExecRequest::ok(data.id, "/execute/start", String::new()).await.unwrap();
    Status::send(req).await;
}

pub async fn display_run_execute(data: ExecReply) {
    println!("data = {}", data.status);
    let execute: Execute = serde_json::from_str(data.status.as_str()).unwrap();
    match run_command(execute.execute_id, execute.command).await {
        Ok(res) => {
            let req = ExecRequest::ok(data.id, "/execute/run", json!(res).to_string()).await.unwrap();
            Status::send(req).await;
        }
        Err(_) => {
            let req = ExecRequest::fail(data.id, "/execute/run", String::new()).await.unwrap();
            Status::send(req).await;
        }
    }
}