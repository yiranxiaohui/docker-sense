use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Grpc {
    pub instance_id: Option<i64>,
    pub url: String,
}