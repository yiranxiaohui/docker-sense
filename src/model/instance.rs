use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub id: i64,
    ip: String,
    status: Status,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Normal,
    Online
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            id: 0,
            ip: local_ip().unwrap().to_string(),
            status: Status::Normal,
        }
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}