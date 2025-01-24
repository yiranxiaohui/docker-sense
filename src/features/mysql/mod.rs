// mod table;
//
// use std::cell::LazyCell;
// use std::sync::{Arc, OnceLock};
// use std::time::Duration;
// use log::{error, info};
// use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
// use sqlx::MySqlPool;
// use tokio::sync::Mutex;
// use crate::config;
//
// pub static MYSQL_POOL: OnceLock<Arc<Mutex<MySqlPool>>> = OnceLock::new();
//
// pub async fn init() {
//     if let Ok(pool) = get_mysql_pool().await {
//         info!("初始化Mysql连接成功！");
//         MYSQL_POOL.get_or_init(|| Arc::new(Mutex::new(pool)));
//     }
// }
//
// pub async fn get_mysql_pool() -> Result<MySqlPool, String> {
//     let mysql = config::init().get_mysql();
//     let options = MySqlConnectOptions::new()
//         .host(mysql.host.as_str())
//         .port(mysql.port)
//         .database(mysql.database.as_str())
//         .username(mysql.username.as_str())
//         .password(mysql.password.as_str());
//     match MySqlPoolOptions::new()
//         .max_connections(5)
//         .acquire_timeout(Duration::from_secs(5))
//         .connect_with(options).await {
//         Ok(pool) => Ok(pool),
//         Err(err) => {
//             error!("连接witdata数据库失败！===> {}", err);
//             Err(String::new())
//         },
//     }
// }