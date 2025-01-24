use lapin::{BasicProperties, Channel};
use lapin::options::BasicPublishOptions;
use log::{error, info};
use crate::features::rabbitmq::model::HandleStatus;

pub mod instance;

pub async fn send(channel: &Channel, queue_name: &str, handle_status: HandleStatus) {
    if let Ok(data) = serde_json::to_string(&handle_status) {
        match channel.basic_publish("",
                                    queue_name,
                                    BasicPublishOptions::default(),
                                    data.as_bytes(),
                                    BasicProperties::default()
        ).await {
            Ok(_) => {
                info!("发送ID = {}的消息成功！", handle_status.id);
            }
            Err(err) => {
                error!("发送ID = {}的消息失败！", handle_status.id);
                error!("error = {}", err);
            }
        }
    }
}