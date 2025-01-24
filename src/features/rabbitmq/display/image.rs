use crate::features::docker::image::{get_image_list, pull_image, remove_image};
use crate::features::docker::model::Image;
use crate::features::rabbitmq::model::{Handle, HandleStatus};
use crate::features::rabbitmq::producer::instance::send;

pub async fn display_get_image_list(handle_status: HandleStatus) {
    match get_image_list().await {
        Ok(res) => {
            let mut data = vec![];
            res.iter().for_each(|i| {
                let image = Image::build(i.clone());
                data.push(image);
            });
            let handle_status= HandleStatus::new(handle_status.id, Handle::GetImageList).set_data(data);
            send("/server/recv", handle_status).await;
        }
        Err(_) => {}
    }
}

pub async fn display_remove_image(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(data)= serde_json::from_str::<Image>(data.as_str()) {
            match remove_image(data.id.unwrap()).await {
                Ok(_) => {
                    let handle_status= HandleStatus::new(handle_status.id, Handle::RemoveImage);
                    send("/server/recv", handle_status).await;
                }
                Err(_) => {}
            }
        }
    }
}

pub async fn display_pull_image(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(data) = serde_json::from_str::<Image>(data.as_str()) {
            pull_image(data.tag.unwrap()).await;
            let handle_status= HandleStatus::new(handle_status.id, Handle::PullImage);
            send("/server/recv", handle_status).await;
        }
    }
}