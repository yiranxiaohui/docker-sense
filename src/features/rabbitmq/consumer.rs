use crate::features::rabbitmq::display::display;
use crate::features::rabbitmq::model::HandleStatus;
use crate::RUNTIME;
use lapin::message::DeliveryResult;
use lapin::options::BasicAckOptions;
use lapin::ConsumerDelegate;
use log::{debug, info};
use std::future::Future;
use std::pin::Pin;

pub struct InstanceConsumer;

impl ConsumerDelegate for InstanceConsumer {
    fn on_new_delivery(&self, delivery: DeliveryResult) -> Pin<Box<dyn Future<Output=()> + Send>> {
        Box::pin(async {
            if let Ok(Some(delivery)) = delivery {
                // 处理消息
                if let Ok(data) = String::from_utf8(delivery.data.clone()) {
                    debug!("{}", data);
                    if let Ok(handle_status) = serde_json::from_str::<HandleStatus>(data.as_str()) {
                        info!("Received message: {:?}", handle_status);
                        // 手动确认消息
                        if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                            eprintln!("Failed to ack message: {:?}", e);
                        }
                        RUNTIME.spawn(display(handle_status));
                    }
                }
            }
        })
    }
}