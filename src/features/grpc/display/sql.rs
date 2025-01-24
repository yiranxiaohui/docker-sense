use serde::Deserialize;
use crate::features::docker::exec::{DockerExec, EXEC};
use crate::features::grpc::gen::docker::ExecReply;

#[derive(Deserialize)]
struct Sql {
    pub name: String,
}

pub async fn create_instance(data: ExecReply) {
    let sql: Sql = serde_json::from_str(data.status.as_str()).unwrap();
    match DockerExec::create_exec(data.id, sql.name).await {
        Ok(_) => {}
        Err(_) => {}
    };
}

pub async fn execute_sql(data: ExecReply) {
    let sql: Sql = serde_json::from_str(data.status.as_str()).unwrap();
    EXEC.clone().lock().await.session.get(&data.id);
}

