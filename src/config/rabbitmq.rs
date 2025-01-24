use std::time::Duration;
use lapin::{Connection, ConnectionProperties};
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RabbitMQ {
    pub addr: String
}

impl RabbitMQ {
    pub async fn get_connect(&self) -> Connection {
        let properties = ConnectionProperties {
            ..Default::default()
        };
        loop {
            match Connection::connect(
                self.addr.as_str(),
                properties.clone()
            ).await {
                Ok(conn) => {
                    info!("Connected to RabbitMQ!");
                    return conn;
                }
                Err(err) => {
                    error!("error => {}", err);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}
