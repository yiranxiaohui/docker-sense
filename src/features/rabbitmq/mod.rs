mod consumer;
pub mod model;
mod display;
pub mod producer;

use std::sync::{Arc, LazyLock, OnceLock, };
use lapin::{Channel, Connection};
use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;
use log::{debug, error, info};
use tokio::spawn;
use tokio::sync::{Mutex, Notify};
use crate::config;
use crate::features::rabbitmq::producer::instance;

pub static RABBITMQ_CONN: OnceLock<Arc<Mutex<RabbitConnection>>> = OnceLock::new();
pub static RABBITMQ_NOTIFY: LazyLock<Arc<Notify>> = LazyLock::new(|| {
   Arc::new(Notify::new())
});
#[derive(Debug)]
pub struct RabbitConnection {
    pub connection: Connection
}

pub async fn init() {
    let mut reconnection = false;
    loop {
        set_conn().await;
        on_error().await;
        if !reconnection {
            spawn(instance::init());
        }
        RABBITMQ_NOTIFY.notified().await;
        info!("重新连接中！");
        reconnection = true;
    }
}

pub async fn create_queue(channel: &Channel, queue_name: String) -> Result<(), String> {
    match channel.queue_declare(queue_name.as_str(),
                          QueueDeclareOptions::default(),
                          FieldTable::default()
    ).await {
        Ok(_ok) => {
            debug!("创建Queue成功！");
            Ok(())
        }
        Err(err) => {
            error!("声明队列{}失败！", queue_name);
            error!("error => {}", err);
            Err(String::new())
        }
    }
}

pub async fn set_conn() {
    let config = config::init();
    let rabbitmq = config.get_rabbitmq();
    let connection = rabbitmq.get_connect().await;
    info!("正在设置Rabbit全局连接！");
    match RABBITMQ_CONN.get() {
        Some(ok) => {
            let mut lock = ok.lock().await;
            match lock.connection.close(200, "结束！").await {
                Ok(_) => {}
                Err(err) => {
                    error!("error => {}", err);
                }
            }
            lock.connection = connection;
        }
        None => {
            RABBITMQ_CONN.set(Arc::new(Mutex::new(RabbitConnection { connection }))).expect("设置失败！");
        }
    }
    info!("设置Rabbit全局连接成功！");
}

pub async fn on_error() {
    if let Some(conn) = RABBITMQ_CONN.get() {
        let conn = &conn.lock().await.connection;
        conn.on_error(|e| {
            error!("error => {}", e);
            RABBITMQ_NOTIFY.notify_one();
            info!("通知成功！");
        });
    }
}

#[tokio::test]
pub async fn test() {
    spawn(move || {
        let mut i = 0;
        while i <= 10 {
            println!("i => {}", i);
            i += 1;
        }
        RABBITMQ_NOTIFY.notify_one();
    });
    RABBITMQ_NOTIFY.notified().await;
    RABBITMQ_NOTIFY.notified().await;
}