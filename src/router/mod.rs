// use axum::Router;
// use crate::config::CONFIG;
//
// pub async fn init() {
//     let app = Router::new();
//     let addr: String = format!("0.0.0.0:{}", CONFIG.axum.port);
//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }