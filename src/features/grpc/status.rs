use std::os::linux::raw::stat;
use std::sync::{Arc, OnceLock};
use log::{error, info};
use tokio::sync::{Mutex, MutexGuard};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use crate::config;
use crate::features::grpc::gen::docker::{ExecReply, ExecRequest};

pub static STATUS: OnceLock<Arc<Mutex<Status>>> = OnceLock::new();

#[derive(Debug)]
pub struct Status {
    // 实例ID
    pub instance_id: i64,
    pub sender: Option<Sender<ExecRequest>>
}

impl Status {
    pub async fn init() {
        let grpc = config::init().get_grpc();
        match STATUS.set(Arc::new(Mutex::new(Status { instance_id: grpc.instance_id.unwrap(), sender: None }))) {
            Ok(ok) => {}
            Err(err) => {
                error!("error => {:?}", err);
            }
        }
    }

    pub async fn get_status_sender() -> Result<Sender<ExecRequest>,String> {
        return match STATUS.get() {
            None => {
                error!("status为空！");
                Err(String::new())
            }
            Some(res) => {
                let clone = res.clone();
                let status = clone.lock().await;
                if let Some(sender) = status.sender.clone() {
                    return Ok(sender);
                }
                Err(String::new())
            }
        }
    }

    pub async fn get_instance_id() -> Result<i64, String>{
        match STATUS.get() {
            None => {
                error!("status_id为空！");
                Err(String::new())
            }
            Some(status) => {
                let instance_id = status.clone().lock().await.instance_id;
                Ok(instance_id)
            }
        }
    }

    pub async fn set_instance_id(instance_id: i64) {
        match STATUS.get() {
            None => {}
            Some(status) => {
                let mut status = status.lock().await;
                status.instance_id = instance_id;
            }
        }
    }

    pub async fn set_sender(tx: Sender<ExecRequest>){
        match STATUS.get() {
            None => {
                error!("sender设置失败！");
            }
            Some(status) => {
                let status_clone = status.clone();
                let mut status = status_clone.lock().await;
                status.sender = Some(tx.clone());
            }
        }
    }

    pub async fn send(exec_request: ExecRequest) {
        if let Ok(sender) = Status::get_status_sender().await {
            match sender.send(exec_request.clone()).await {
                Ok(ok) => {
                    info!("会话{}发送成功！", exec_request.id);
                }
                Err(err) => {
                    error!("发送失败！");
                }
            }
        }
    }
}