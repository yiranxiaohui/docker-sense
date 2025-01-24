// use futures::{FutureExt, SinkExt, StreamExt};
// use std::sync::{Arc, OnceLock};
// use std::time::Duration;
// use log::{error, info};
// use serde_json::{json, Value};
// use tokio::spawn;
// use tokio::sync::{mpsc, Mutex};
// use tokio::sync::mpsc::error::SendError;
// use tokio::sync::mpsc::Sender;
// use tokio::time::sleep;
// use tokio_stream::wrappers::ReceiverStream;
// use tonic::transport::{Channel, Error};
// use tonic::{IntoRequest, IntoStreamingRequest, Request, Streaming};
// use crate::config;
// use crate::features::grpc::display::display;
// use crate::features::grpc::gen::docker::docker_client::DockerClient;
// use crate::features::grpc::gen::docker::ExecRequest;
// use crate::features::grpc::status::Status;
//
// pub mod gen;
// mod display;
// mod check;
// mod exec;
// pub mod status;
//
// pub async fn get_client() -> DockerClient<Channel> {
//     loop {
//         let grpc = config::init().get_grpc();
//         match DockerClient::connect(grpc.url).await {
//             Ok(client) => {
//                 info!("Grpc初始化成功！");
//                 return client;
//             }
//             Err(err) => {
//                 error!("error = {}", err.to_string());
//             }
//         }
//         sleep(Duration::from_secs(1)).await;
//     }
// }
//
// pub async fn init() {
//     Status::init().await;
//     loop {
//         let mut client = get_client().await;
//         let check_handle = spawn(check::init(client.clone()));
//         sleep(Duration::from_secs(1)).await;
//         let exec_handle = spawn(exec::init(client.clone()));
//         match check_handle.await {
//             Ok(_) => {}
//             Err(err) => {
//                 break;
//             }
//         }
//     }
// }