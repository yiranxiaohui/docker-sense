use log::debug;

use crate::config::{self, edit_instance_id};
use crate::features::rabbitmq::model::{Handle, HandleStatus};
use crate::features::rabbitmq::producer::instance::{send, INSTANCE_NOTIFY};
use crate::model::instance::{Instance, Status};

pub async fn display_register_instance(handle_status: HandleStatus) {
    if let Some(data) = handle_status.data.get(0) {
        if let Ok(data) = data.parse() {
            edit_instance_id(data);
            INSTANCE_NOTIFY.notify_one();
        }
    }
}

pub async fn display_heartbeat(handle_status: HandleStatus) {
    let config = config::init();
    let instance_id = config.instance_id;
    if let Some(instance_id) = instance_id {
        let mut instance = Instance::new();
        instance.set_id(instance_id);
        instance.set_status(Status::Online);
        let handle_status = HandleStatus::new(handle_status.id, Handle::Heartbeat)
            .set_data(instance);
        send("/server/recv", handle_status).await;
        debug!("发送心跳成功！");
    }
}