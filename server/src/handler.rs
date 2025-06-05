use axum::{body::Bytes, http::StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};

pub async fn unit_handler() {}

pub async fn string_handler() -> String {
    "Hello, World".to_string()
}

pub async fn get_data() -> Json<Value> {
    Json(json!({"message": "Hello from Axum!"}))
}