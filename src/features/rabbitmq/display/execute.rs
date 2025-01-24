use serde::Deserialize;

use crate::features::{docker::{execute::{create_execute, run_command, start_execute}, model::Container}, rabbitmq::{model::{Handle, HandleStatus}, producer::instance::send}};

#[derive(Deserialize)]
struct Execute {
    id: String,
    command: Option<String>
}

pub async fn display_create_execute(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(container)= serde_json::from_str::<Container>(data.as_str()) {
            match create_execute(container.id.unwrap()).await {
                Ok(res) => {
                    let mut handle_status= HandleStatus::new(handle_status.id, Handle::CreateContainer);
                    handle_status.data = vec![res.id];
                    send("/server/recv", handle_status).await;
                }
                Err(_) => {}
            };
        }
    }
}

pub async fn display_start_execute(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(execute)= serde_json::from_str::<Execute>(data.as_str()) {
            match start_execute(execute.id, handle_status.id).await {
                Ok(_) => {
                    let handle_status= HandleStatus::new(handle_status.id, Handle::StartExecute);
                    send("/server/recv", handle_status).await;
                },
                Err(_) => todo!(),
            }
        }
    }
}

pub async fn display_run_execute(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(execute)= serde_json::from_str::<Execute>(data.as_str()) {
            run_command(execute.id, execute.command.unwrap()).await;
        }
    }
}