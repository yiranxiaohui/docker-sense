use bollard::models::{ContainerSummary, ImageSummary};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Container {
    pub id: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub state: Option<String>,
    pub image_id: Option<String>,
    pub created: Option<i64>,
}

impl Container {
    pub fn build(container_summary: ContainerSummary) -> Self {
        Container {
            id: container_summary.id,
            name: container_summary.names.unwrap().first().map(|i| i.clone()),
            image: container_summary.image,
            state: container_summary.state,
            image_id: container_summary.image_id,
            created: container_summary.created,
        }
    }

}

#[derive(Deserialize, Debug, Serialize)]
pub struct Image {
    pub id: Option<String>,
    pub tag: Option<String>,
    pub digest: Option<String>,
    pub created: Option<i64>,
    pub size: Option<i64>,
}

impl Image {
    pub fn build(image_summary: ImageSummary) -> Self {
        Image {
            id: Some(image_summary.id),
            tag: image_summary.repo_tags.first().map(|i| i.clone()),
            digest: image_summary.repo_digests.first().map(|i| i.clone()),
            created: Some(image_summary.created),
            size: Some(image_summary.size),
        }
    }
}