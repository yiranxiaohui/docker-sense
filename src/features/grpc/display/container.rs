use bollard::models::ContainerSummary;
use log::{error, info};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::features::docker::container::{create_container, get_container_list, remove_container, start_container, stop_container};
use crate::features::grpc::gen::docker::{ExecReply, ExecRequest};
use crate::features::grpc::status::Status;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub id: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
}

pub async fn display_create(data: ExecReply) {
    let create: Container = serde_json::from_str(data.status.as_str()).unwrap();
    let res = create_container(create.image.unwrap(), create.name.unwrap()).await.unwrap();
    let req = ExecRequest::ok(data.id, "/container/create", json!(res).to_string()).await.unwrap();
    Status::send(req).await;
}

pub async fn display_start(data: ExecReply) {
    let container: Container = serde_json::from_str(data.status.as_str()).unwrap();
    match start_container(container.id.unwrap()).await {
        Ok(_) => {
            let req = ExecRequest::ok(data.id, "/container/start", String::new()).await.unwrap();
            Status::send(req).await;
        }
        Err(_) => {}
    }
}

pub async fn display_stop(data: ExecReply) {
    let container: Container = serde_json::from_str(data.status.as_str()).unwrap();
    match stop_container(container.id.unwrap()).await {
        Ok(_) => {
            let req = ExecRequest::ok(data.id, "/container/stop", String::new()).await.unwrap();
            Status::send(req).await;
        }
        Err(_) => {}
    }
}

pub async fn display_remove(data: ExecReply) {
    let container: Container = serde_json::from_str(data.status.as_str()).unwrap();
    match remove_container(container.id.unwrap()).await {
        Ok(_) => {
            let req = ExecRequest::ok(data.id, "/container/remove", String::new()).await.unwrap();
            Status::send(req).await;
        }
        Err(_) => {}
    }
}

pub async fn display_container_list(data: ExecReply) {
    match get_container_list().await {
        Ok(res) => {
            let req = ExecRequest::ok(data.id, "/container/list", serde_json::to_string(&res).unwrap_or(String::new())).await.unwrap();
            Status::send(req).await;
        }
        Err(_) => {}
    }
}