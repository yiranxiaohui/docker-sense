use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use bollard::container::LogOutput;
use bollard::errors::Error;
use bollard::exec::{CreateExecOptions, CreateExecResults, StartExecOptions, StartExecResults};
use bollard::exec::StartExecResults::Attached;
use futures::TryStreamExt;
use log::{error, info};
use serde_json::json;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_stream::Stream;
use crate::features::docker::get_docker;
use crate::features::rabbitmq::model::{Handle, HandleStatus};
use crate::features::rabbitmq::producer::instance::send;

pub static EXECUTE_MAP: LazyLock<Arc<Mutex<HashMap<String, Execute>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub struct Execute {
    output: Arc<Mutex<Pin<Box<dyn Stream<Item = Result<LogOutput, Error>> + Send>>>>,
    input: Arc<Mutex<Pin<Box<dyn AsyncWrite + Send>>>>,
}

pub async fn create_execute(container_id: String) -> Result<CreateExecResults, ()>{
    if let Ok(docker) = get_docker() {
        let options = CreateExecOptions {
            attach_stdin: Some(true),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            tty: Some(true),
            cmd: Some(vec!["bash"]),
            ..Default::default()
        };
        return match docker.create_exec(container_id.as_str(), options).await {
            Ok(res) => {
                Ok(res)
            }
            Err(err) => {
                error!("容器{}创建终端失败！", container_id);
                error!("error = {}", err);
                Err(())
            }
        }
    };
    Err(())
}

pub async fn start_execute(execute_id: String, handle_status_id: i64) -> Result<(), ()>{
    if let Ok(docker) = get_docker() {
        let option = StartExecOptions {
            tty: true,
            ..Default::default()
        };
        match docker.start_exec(execute_id.as_str(), Some(option)).await {
            Ok(res) => {
                match res {
                    Attached { output, input } => {
                        let output = Arc::new(Mutex::new(output));
                        let execute = Execute {
                            output: output.clone(),
                            input: Arc::new(Mutex::new(input))
                        };
                        let clone = EXECUTE_MAP.clone();
                        spawn(recv_data(handle_status_id, execute_id.clone(), execute.output.clone()));
                        let mut map = clone.lock().await;
                        match map.insert(execute_id, execute) {
                            None => {
                                info!("插入成功！");
                            }
                            Some(_) => {}
                        };
                        return Ok(());
                    }
                    StartExecResults::Detached => {
                        error!("解析res失败！");
                    }
                }
            }
            Err(err) => {
                error!("启动失败！");
                error!("error => {}", err);
            }
        }
    }
    Err(())
}

pub async fn run_command(execute_id: String, command: String) {
    let clone = EXECUTE_MAP.clone();
    let map = clone.lock().await;
    let command = command+"\n";
    if let Some(execute) = map.get(&execute_id) {
        let input = execute.input.clone();
        let mut input = input.lock().await;
        match input.write(command.as_bytes()).await {
            Ok(_) => {}
            Err(err) => {
                error!("终端{}写入失败！", execute_id);
                error!("error => {}", err);
            }
        }
    }
}

pub async fn recv_data(handle_status_id: i64, execute_id: String, output: Arc<Mutex<Pin<Box<dyn Stream<Item = Result<LogOutput, Error>> + Send>>>>) {
    loop {
        let mut output = output.lock().await;
        match output.try_next().await {
            Ok(Some(res)) => {
                match res {
                    LogOutput::StdErr { message } => {
                        let message = String::from_utf8(message.to_vec()).unwrap();
                        error!("stderr => {}", message);
                    }
                    LogOutput::StdOut { message } => {
                        let message = String::from_utf8(message.to_vec()).unwrap();
                        info!("stdout = {}", message);
                    }
                    LogOutput::StdIn { message } => {
                        let message = String::from_utf8(message.to_vec()).unwrap();
                        error!("stdin => {}", message);
                    }
                    LogOutput::Console { message } => {
                        let message = String::from_utf8(message.to_vec()).unwrap();
                        let mut handle_status= HandleStatus::new(handle_status_id, Handle::RunCommand);
                        handle_status.data.push(json!({
                            "id": execute_id,
                            "output": message
                        }).to_string());
                        handle_status.data.push(message);
                        send("/server/recv", handle_status).await;
                    }
                }
            }
            Ok(None) => {
                // error!("消息为空！");
            }
            Err(err) => {
                error!("error => {}", err);
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}