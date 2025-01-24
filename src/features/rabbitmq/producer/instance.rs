use crate::features::rabbitmq::model::{Handle, HandleStatus};
use crate::features::rabbitmq::{consumer, create_queue, RABBITMQ_CONN};
use crate::model::instance::Instance;
use lapin::{BasicProperties, Channel};
use std::sync::{Arc, LazyLock, OnceLock};
use lapin::options::{BasicConsumeOptions, BasicPublishOptions};
use lapin::types::FieldTable;
use log::{debug, error, info};
use tokio::sync::{Mutex, Notify};
use crate::config;

pub static INSTANCE_CHANNEL: OnceLock<Arc<Mutex<InstanceChannel>>> = OnceLock::new();
pub static INSTANCE_NOTIFY: LazyLock<Notify> = LazyLock::new(|| {
    Notify::new()
});
#[derive(Debug)]
pub struct InstanceChannel {
    pub channel: Channel
}

pub async fn init() {
    loop {
        let channel = create_channel().await;
        set_channel(channel).await;
        subscribe().await;
        check_instance_id().await;
        info!("InstanceChannel初始化成功！");
        on_error().await;
        INSTANCE_NOTIFY.notified().await;
    }
}

pub async fn set_channel(channel: Channel) {
    match INSTANCE_CHANNEL.get() {
        None => {
            INSTANCE_CHANNEL.set(Arc::new(Mutex::new(InstanceChannel { channel }))).expect("设置HEALTH_CHANNEL失败！");
        }
        Some(instance_channel) => {
            let mut lock = instance_channel.lock().await;
            match lock.channel.close(200, "结束！").await {
                Ok(_ok) => {}
                Err(err) => {
                    error!("error => {}", err);
                }
            }
            lock.channel = channel;
        }
    }
}

pub async fn create_channel() -> Channel {
    loop {
        if let Some(conn) = RABBITMQ_CONN.get() {
            let conn = &conn.lock().await.connection;
            match conn.create_channel().await {
                Ok(channel) => {
                    return channel;
                }
                Err(err) => {
                    error!("为Instance创建Channel失败!");
                    error!("error => {}", err);
                }
            };
        }
    }
}

pub async fn on_error(){
    if let Some(channel) = INSTANCE_CHANNEL.get() {
        let channel = &channel.lock().await.channel;
        channel.on_error(move |err| {
            info!("error => {}", err);
            INSTANCE_NOTIFY.notify_one();
            info!("通知成功！");
        });
    }
}

pub async fn send(queue_name: &str, handle_status: HandleStatus) {
    if let Some(channel) = INSTANCE_CHANNEL.get() {
        let channel = &channel.lock().await.channel;
        if let Ok(data) = serde_json::to_string(&handle_status) {
            match channel.basic_publish("",
                                        queue_name,
                                        BasicPublishOptions::default(),
                                        data.as_bytes(),
                                        BasicProperties::default()
            ).await {
                Ok(_) => {
                    debug!("发送ID = {}的消息成功！", handle_status.id);
                }
                Err(err) => {
                    error!("发送ID = {}的消息失败！", handle_status.id);
                    error!("error = {}", err);
                }
            }
        }
    }
}

pub async fn subscribe() {
    if let Some(channel) = INSTANCE_CHANNEL.get() {
        let config = config::init();
        let instance_id = config.instance_id;
        let instance_id = match instance_id {
            Some(res) => res,
            None => 0,
        };
        let channel = &channel.lock().await.channel;
        let queue_name = format!("/instance/{}", instance_id);
        create_queue(&channel, queue_name.clone()).await.expect("创建队列失败！");
        let consumer = channel.basic_consume(
            queue_name.clone().as_str(),
            "",
            BasicConsumeOptions::default(),
            FieldTable::default()
        ).await.expect("创建消费者失败！");
        info!("订阅{}成功！", queue_name);
        consumer.set_delegate(consumer::InstanceConsumer);
    }
}

pub async fn check_instance_id() -> bool {
    let config = config::init();
    let instance_id = config.instance_id;
    match instance_id {
        None => {
            get_instance_id().await;
            false
        }
        Some(_instance_id) => {
            true
        }
    }
}

pub async fn get_instance_id(){
    let handle_status = HandleStatus::new(0, Handle::RegisterInstance).set_data(Instance::new());
    send( "/server/recv", handle_status).await;
}