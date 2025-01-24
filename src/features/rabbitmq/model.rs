use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct HandleStatus {
    pub id: i64,
    pub status: Option<Status>,
    pub handle: Handle,
    pub data: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Handle {
    GetContainerList,
    CreateContainer,
    StartContainer,
    StopContainer,
    RemoveContainer,
    GetImageList,
    RemoveImage,
    PullImage,
    RegisterInstance,
    CheckInstance,
    GetConfig,
    Other,
    Heartbeat,
    CreateExecute,
    StartExecute,
    RunCommand,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    Waiting,
    Success,
    Fail,
    Unknown,
}

impl HandleStatus {
    pub fn new(id: i64, handle: Handle) -> Self {
        HandleStatus {
            id,
            status: Some(Status::Success),
            handle,
            data: Vec::new()
        }
    }

    pub fn set_data <T: Serialize>(mut self, data: T) -> HandleStatus {
        if let Ok(data) = serde_json::to_string(&data) {
            self.data.push(data);
        }
        self
    }

    pub fn set_status(mut self, status: Status) {
        self.status = Some(status);
    }
}

impl FromStr for Handle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/create/container" => {
                Ok(Handle::CreateContainer)
            }
            _ => {
                Ok(Handle::Other)
            }
        }
    }
}

