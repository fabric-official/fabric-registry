
use axum::{routing::get, Router, response::IntoResponse};
use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
struct Model { id: String, name: String, version: String }

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/", get(root))
        .route("/models", get(list_models))
        .route("/manifests/validate", get(validate_manifest));

    let addr = SocketAddr::from(([0,0,0,0], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn health() -> impl IntoResponse { axum::Json(serde_json::json!({"status":"ok"})) }
async fn root() -> impl IntoResponse { axum::Json(serde_json::json!({"service":"registry"})) }
async fn list_models() -> impl IntoResponse {
    axum::Json(vec![ Model{ id:"assistive_vision".into(), name:"assistive_vision".into(), version:"1.0.0".into() } ])
}
async fn validate_manifest() -> impl IntoResponse {
    axum::Json(serde_json::json!({"valid":true}))
}
