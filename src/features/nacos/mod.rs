// use nacos_sdk::api::naming::{NamingService, ServiceInstance};
// use std::sync::{Arc, OnceLock};
// use log::{error, info};
// use nacos_sdk::api::constants;
// use tokio::sync::Mutex;
//
// pub static NAMING: OnceLock<Arc<Mutex<dyn NamingService>>> = OnceLock::new();
//
// pub async fn init() {
//     let naming_service = nacos_sdk::api::naming::NamingServiceBuilder::new(
//         nacos_sdk::api::props::ClientProps::new()
//             .server_addr("web.yunnet.top:8848")
//             .namespace("")
//             .auth_username("nacos")
//             .auth_password("nacos")
//             .app_name("docker"),
//     ).enable_auth_plugin_http().build().unwrap();
//     NAMING.get_or_init(|| Arc::new(Mutex::new(naming_service)));
//     let service_instance1 = ServiceInstance {
//         ip: "0.0.0.0".to_string(),
//         port: 9090,
//         ..Default::default()
//     };
//     let _register_instance_ret = NAMING.get().unwrap().lock().await
//         .batch_register_instance(
//             "test-service".to_string(),
//             Some(constants::DEFAULT_GROUP.to_string()),
//             vec![service_instance1],
//         )
//         .await;
//     tokio::time::sleep(tokio::time::Duration::from_millis(666)).await;
//
//     let instances_ret = NAMING.get().unwrap().lock().await
//         .get_all_instances(
//             "test-service".to_string(),
//             Some(constants::DEFAULT_GROUP.to_string()),
//             Vec::default(),
//             false,
//         )
//         .await;
//     match instances_ret {
//         Ok(instances) => info!("get_all_instances {:?}", instances),
//         Err(err) => error!("naming get_all_instances error {:?}", err),
//     }
// }