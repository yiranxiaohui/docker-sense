use log::{error, info};
use serde_json::{json, Value};
use crate::features::grpc::display::display;
use crate::features::grpc::gen::docker::docker_client::DockerClient;
use crate::features::grpc::status::Status;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use tonic::{Request, Response, Streaming};
use crate::features::grpc::gen::docker::{ExecReply, ExecRequest};

impl ExecRequest {
    pub async fn ok(id: i64, r#type: &str, status: String) -> Result<ExecRequest, String> {
        if let Ok(instance_id) = Status::get_instance_id().await {
            return Ok(ExecRequest {
                instance_id,
                id,
                code: 200,
                msg: "".to_string(),
                r#type: r#type.to_string(),
                status,
            });
        };
        error!("获取实例ID失败！");
        Err(String::new())
    }

    pub async fn fail(id: i64, r#type: &str, status: String) -> Result<ExecRequest, String> {
        if let Ok(instance_id) = Status::get_instance_id().await {
            return Ok(ExecRequest {
                instance_id,
                id,
                code: 500,
                msg: "".to_string(),
                r#type: r#type.to_string(),
                status,
            });
        };
        error!("获取实例ID失败！");
        Err(String::new())
    }
}

pub async fn init(mut client: DockerClient<Channel>) {
    let (tx, rx) = mpsc::channel(1000);
    Status::set_sender(tx).await;
    match client.exec(Request::new(ReceiverStream::new(rx))).await {
        Ok(res) => {
            Status::send(ExecRequest::ok(0, "hello", String::new()).await.unwrap()).await;
            spawn(async move {
                let mut res_stream = res.into_inner();
                while let Some(item) = res_stream.next().await {
                    if let Ok(reply) = item {
                        spawn(display(reply));
                    }
                }
            });
        }
        Err(err) => {
            error!("err = {:?}", err);
        }
    }
}