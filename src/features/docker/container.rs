use bollard::container::{Config, CreateContainerOptions, ListContainersOptions, RemoveContainerOptions, StartContainerOptions, StopContainerOptions};
use bollard::models::{ContainerCreateResponse, ContainerSummary};
use log::{error, info};
use crate::features::docker::get_docker;

pub async fn create_container(image: String, name: String) -> Result<ContainerCreateResponse, String> {
    if let Ok(docker) = get_docker() {
        let options = Some(CreateContainerOptions {
            name,
            ..Default::default()
        });
        let config = Config {
            image: Some(image),
            env: Some(vec!["MYSQL_ALLOW_EMPTY_PASSWORD=yes".to_string()]),
            ..Default::default()
        };
        return match docker.create_container(options, config).await {
            Ok(res) => {
                Ok(res)
            }
            Err(err) => {
                println!("error => {:?}", err);
                error!("创建容器失败！");
                error!("error = {:?}", err);
                Err(String::new())
            }
        }
    }
    Err(String::new())
}

pub async fn start_container(id: String) -> Result<(), String> {
    if let Ok(docker) = get_docker() {
        return match docker.start_container(id.as_str(), None::<StartContainerOptions<String>>).await {
            Ok(_res) => {
                Ok(())
            }
            Err(err) => {
                error!("启动容器失败！");
                error!("error = {:?}", err);
                Err(String::new())
            }
        };
    }
    Err(String::new())
}

pub async fn stop_container(id: String) -> Result<(), String> {
    if let Ok(docker) = get_docker() {
        return match docker.stop_container(id.as_str(), None::<StopContainerOptions>).await {
            Err(err) => {
                error!("停止容器失败！");
                error!("error = {:?}", err);
                Err(String::new())
            }
            Ok(_) => {
                info!("停止容器成功！");
                Ok(())
            }
        };
    }
    Err(String::new())
}

pub async fn remove_container(id: String) -> Result<(), String> {
    if let Ok(docker) = get_docker() {
        return match docker.remove_container(id.as_str(), None::<RemoveContainerOptions>).await {
            Err(err) => {
                error!("删除容器失败！");
                error!("error = {:?}", err);
                Err(String::new())
            }
            Ok(_) => {
                info!("删除容器{}成功！", id);
                Ok(())
            }
        };
    }
    Err(String::new())
}

pub async fn get_container_list() -> Result<Vec<ContainerSummary>, String> {
    if let Ok(docker) = get_docker() {
        let options: ListContainersOptions<String> = ListContainersOptions {
            all: true,
            limit: None,
            size: false,
            ..Default::default()
        };
        return match docker.list_containers(Some(options)).await {
            Err(err) => {
                error!("获取容器失败！");
                error!("error = {:?}", err);
                Err(String::new())
            }
            Ok(res) => {
                Ok(res)
            }
        };
    }
    Err(String::new())
}

