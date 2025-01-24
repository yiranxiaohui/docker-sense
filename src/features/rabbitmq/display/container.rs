use crate::features::docker::container::{create_container, get_container_list, remove_container, start_container, stop_container};
use crate::features::docker::model::Container;
use crate::features::rabbitmq::model::{Handle, HandleStatus};
use crate::features::rabbitmq::producer::instance::send;

pub async fn display_get_container_list(handle_status: HandleStatus) {
    match get_container_list().await {
        Ok(res) => {
            let mut data = vec![];
            res.iter().for_each(|i| {
                data.push(Container::build(i.clone()));
            });
            let handle_status= HandleStatus::new(handle_status.id, Handle::CreateContainer).set_data(data);
            send("/server/recv", handle_status).await;
        }
        Err(_) => {}
    };
}

pub async fn display_create_container(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(data)= serde_json::from_str::<Container>(data.as_str()) {
            match create_container(data.image.unwrap(), data.name.unwrap()).await {
                Ok(res) => {
                    let handle_status= HandleStatus::new(handle_status.id, Handle::CreateContainer).set_data(res.id);
                    send("/server/recv", handle_status).await;
                }
                Err(_) => {}
            };
        }
    }
}

pub async fn display_start_container(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(data)= serde_json::from_str::<Container>(data.as_str()) {
            match start_container(data.id.unwrap()).await {
                Ok(_) => {
                    let handle_status= HandleStatus::new(handle_status.id, Handle::StartContainer);
                    send("/server/recv", handle_status).await;
                }
                Err(_) => {}
            };
        }
    }
}

pub async fn display_stop_container(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(data)= serde_json::from_str::<Container>(data.as_str()) {
            match stop_container(data.id.unwrap()).await {
                Ok(_) => {
                    let handle_status= HandleStatus::new(handle_status.id, Handle::StopContainer);
                    send("/server/recv", handle_status).await;
                }
                Err(_) => {}
            }
        }
    }
}

pub async fn display_remove_container(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(data)= serde_json::from_str::<Container>(data.as_str()) {
            match remove_container(data.id.unwrap()).await {
                Ok(_) => {
                    let handle_status= HandleStatus::new(handle_status.id, Handle::CreateContainer);
                    send("/server/recv", handle_status).await;
                }
                Err(_) => {
                }
            }
        }
    }
}