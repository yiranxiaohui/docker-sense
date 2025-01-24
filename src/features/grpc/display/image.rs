use crate::features::docker::image::get_image_list;
use crate::features::grpc::gen::docker::{ExecReply, ExecRequest};
use serde::Deserialize;
use serde_json::json;
use crate::features::grpc::status::Status;

#[derive(Deserialize)]
struct Image {
    pub name: String,
}

pub async fn image_list(data: ExecReply) {
    match get_image_list().await {
        Ok(res) => {
            let req = ExecRequest::ok(data.id, "/image/list", json!(res).to_string()).await.unwrap();
            Status::send(req).await;
        }
        Err(err) => {

        }
    }
}