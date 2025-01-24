mod container;
mod image;
mod instance;
mod execute;

use execute::{display_create_execute, display_run_execute, display_start_execute};
use instance::display_heartbeat;

use crate::features::rabbitmq::display::container::{display_create_container, display_get_container_list, display_remove_container, display_start_container, display_stop_container};
use crate::features::rabbitmq::display::image::{display_get_image_list, display_pull_image, display_remove_image};
use crate::features::rabbitmq::display::instance::display_register_instance;
use crate::features::rabbitmq::model::{Handle, HandleStatus};

pub async fn display(data: HandleStatus) {
    match data.handle {
        Handle::Other => {}
        Handle::GetContainerList => {
            display_get_container_list(data).await;
        }
        Handle::CreateContainer => {
            display_create_container(data).await;
        }
        Handle::StartContainer => {
            display_start_container(data).await;
        }
        Handle::StopContainer => {
            display_stop_container(data).await;
        }
        Handle::RemoveContainer => {
            display_remove_container(data).await;
        }
        Handle::GetImageList => {
            display_get_image_list(data).await;
        }
        Handle::RemoveImage => {
            display_remove_image(data).await;
        }
        Handle::PullImage => {
            display_pull_image(data).await;
        }
        Handle::RegisterInstance => {
            display_register_instance(data).await;
        }
        Handle::Heartbeat => {
            display_heartbeat(data).await;
        }
        Handle::CreateExecute => {
            display_create_execute(data).await;
        }
        Handle::StartExecute => {
            display_start_execute(data).await;
        }
        Handle::RunCommand => {
            display_run_execute(data).await;
        }
        _ => {}
    }
}