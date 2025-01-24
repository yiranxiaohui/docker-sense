mod container;
mod image;
mod sql;
mod execute;

use crate::features::grpc::display::container::{display_container_list, display_create, display_remove, display_start, display_stop};
use crate::features::grpc::display::execute::{display_create_execute, display_run_execute, display_start_execute};
use crate::features::grpc::display::image::image_list;
use crate::features::grpc::gen::docker::ExecReply;
use serde::Deserialize;

pub async fn display(data: ExecReply) {
    if data.r#type.starts_with("/container/create") {
        display_create(data).await;
    } else if data.r#type.starts_with("/container/start") {
        display_start(data).await;
    } else if data.r#type.starts_with("/container/stop") {
        display_stop(data).await;
    } else if data.r#type.starts_with("/container/remove") {
        display_remove(data).await;
    } else if data.r#type.starts_with("/container/list") {
        display_container_list(data).await;
    } else if data.r#type.starts_with("/image/list") {
        image_list(data).await;
    } else if data.r#type.starts_with("/execute/create") {
        display_create_execute(data).await;
    } else if data.r#type.starts_with("/execute/start") {
        display_start_execute(data).await;
    } else if data.r#type.starts_with("/execute/run") {
        display_run_execute(data).await;
    }
}