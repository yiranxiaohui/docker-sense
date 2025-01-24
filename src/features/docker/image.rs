use bollard::auth::DockerCredentials;
use bollard::image::{CreateImageOptions, ListImagesOptions, RemoveImageOptions};
use bollard::models::ImageSummary;
use futures::StreamExt;
use log::error;
use crate::features::docker::get_docker;

pub async fn get_image_list() -> Result<Vec<ImageSummary>, String> {
    if let Ok(docker) = get_docker() {
        return match docker.list_images(None::<ListImagesOptions<String>>).await {
            Ok(res) => {
                Ok(res)
            }
            Err(err) => {
                error!("停止容器失败！");
                error!("error = {:?}", err);
                Err(String::new())
            }
        };
    }
    Err(String::new())
}

pub async fn remove_image(id: String) -> Result<(), String> {
    if let Ok(docker) = get_docker() {
        return match docker.remove_image(id.as_str(), None::<RemoveImageOptions>, None::<DockerCredentials>).await {
            Ok(_res) => {
                Ok(())
            }
            Err(err) => {
                error!("删除镜像失败！");
                error!("error => {}", err);
                Err(String::new())
            }
        }
    };
    Err(String::new())
}

pub async fn pull_image(image_name: String) {
    if let Ok(docker) = get_docker() {
        println!("image_name = {}", image_name);
        let options = Some(CreateImageOptions {
            from_image: image_name.as_str(),
            ..Default::default()
        });
        let mut stream = docker.create_image(options, None, None);
        loop {
            match stream.next().await {
                None => {}
                Some(res) => {
                    if let Ok(res) = res {
                        println!("res => {:?}", res);
                    }
                }
            }
        }
    }
}