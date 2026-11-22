use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status" : "ok",
        "message" : "Hello, World!"
    });
    Json(json_response)
}