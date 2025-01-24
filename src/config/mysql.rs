use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Mysql {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}