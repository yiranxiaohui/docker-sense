mod mysql;
mod grpc;
mod rabbitmq;

use std::fs;
use log::info;
use serde::{Deserialize, Serialize};
use crate::config::rabbitmq::RabbitMQ;

#[derive(Deserialize, Debug, Serialize)]
pub struct Axum {
    pub port: i32
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Mqtt {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub topics: Vec<String>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub instance_id: Option<i64>,
    // pub axum: Axum,
    // mqtt: Mqtt,
    rabbitmq: RabbitMQ
}

impl Config {

    pub fn get_rabbitmq(&self) -> RabbitMQ {
        self.rabbitmq.clone()
    }
}


pub fn init() -> Config {
    let content = fs::read_to_string("config.toml").expect("读取配置文件失败，请检查配置文件是否存在！");
    let config: Config = toml::from_str(content.as_str()).expect("读取配置文件失败，请检查配置文件格式是否符合要求！");
    config
}


pub fn edit_instance_id(instance_id: i64) {
    let mut config = init();
    config.instance_id = Some(instance_id);
    info!("instance_id => {}", instance_id);
    let toml = toml::to_string(&config).expect("转换TOML失败！");
    fs::write("config.toml", toml).expect("刷新配置文件失败！");
}