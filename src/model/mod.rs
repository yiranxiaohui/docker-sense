pub(crate) mod instance;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Payload {
    Sql(Sql)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sql {
    pub r#type: String,
    pub sql: String,
}