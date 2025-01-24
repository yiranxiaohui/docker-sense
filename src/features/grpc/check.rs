// use std::os::fd::AsRawFd;
// use std::time::Duration;
// use chrono::{Local, Utc};
// use local_ip_address::local_ip;
// use log::{error, info};
// use serde_json::json;
// use tokio::sync::mpsc;
// use tokio::sync::mpsc::error::SendError;
// use tokio::time::sleep;
// use tokio_stream::StreamExt;
// use tokio_stream::wrappers::ReceiverStream;
// use tonic::{Response, Streaming};
// use tonic::transport::Channel;
// use crate::features;
// use crate::features::grpc;
// use crate::features::grpc::gen::docker::docker_client::DockerClient;
// use crate::features::grpc::gen::docker::{HealthCheckRequest, HealthCheckResponse, ServingStatus};
// use crate::features::grpc::status;
// use crate::features::grpc::status::Status;
//
// impl HealthCheckRequest {
//     pub async fn ok() -> Result<HealthCheckRequest, String> {
//         let ip = local_ip().unwrap();
//         let instance_id = match Status::get_instance_id().await {
//             Ok(instance_id) => {
//                 // 发送实例健康状态
//                 instance_id
//             }
//             Err(err) => {
//                 error!("error => {:?}", err);
//                 0
//             }
//         };
//         let req = HealthCheckRequest {
//             instance_id,
//             status: json!({
//                 "id": instance_id,
//                 "ip": ip.to_string(),
//                 "updated": Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
//             }).to_string(),
//         };
//         return Ok(req);
//     }
// }
//
// pub async fn init(mut client: DockerClient<Channel>) -> Result<String, String> {
//     let (tx, rx) = mpsc::channel(1000);
//     let req_stream = ReceiverStream::new(rx);
//     match client.check(req_stream).await {
//         Ok(res) => {
//             let mut stream = res.into_inner();
//             while let Some(Ok(item)) = stream.next().await {
//                 if let ServingStatus::Init = item.status() {
//                     edit_instance_id(item.instance_id);
//                     Status::set_instance_id(item.instance_id).await;
//                     break;
//                 }
//                 match tx.send(HealthCheckRequest::ok().await?).await {
//                     Ok(res) => {
//                     }
//                     Err(err) => {
//                         break;
//                     }
//                 };
//             }
//         }
//         Err(err) => {
//             error!("err => {:?}", err);
//         }
//     };
//     Err(String::new())
// }