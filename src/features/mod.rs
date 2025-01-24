use tokio::signal;

mod mqtt;
mod mysql;
mod log;
mod nacos;
mod grpc;
mod docker;
mod rabbitmq;

pub async fn init() {
    log::init();
    tokio::spawn(rabbitmq::init());
    signal::ctrl_c().await.expect("Ctrl+C失败！");
    println!("Press Ctrl+C to stop.");
    println!("Received Ctrl+C, stopping the main thread.");
}