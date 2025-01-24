pub mod image;
pub mod container;
pub mod execute;
pub mod model;

use bollard::Docker;
use log::{error};

fn get_docker() -> Result<Docker, String> {
    match Docker::connect_with_local_defaults() {
        Ok(client) => {
            Ok(client)
        }
        Err(err) => {
            error!("连接Docker实例失败！");
            error!("{:?}", err);
            Err(String::new())
        }
    }
}

pub async fn _ping() {
    if let Ok(docker) = get_docker() {
        let message = docker.ping().await.unwrap();
        println!("{}", message);
    }
}