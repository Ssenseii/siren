use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{AppState, model::User, schema::UserSchema};


pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let id = uuid::Uuid::new_v4();

    // todo: password hashing

    let user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (id, username, email, password)
                VALUES ($1::uuid, $2, $3, $4)
                RETURNING *
            "#,
            id,
            body.username,
            body.email,
            body.password
        )
        .fetch_one(&data.db)
        .await
        .map_err(|e| e.to_string());

    if let Err(err) = user {
         if err.to_string().contains("duplicate key value") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "User already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    };

    let user_response = json!({
        "status": "success",
        "data": json!({
                "user": user
                })
            });

    Ok(Json(user_response))
}


pub async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status" : "ok",
        "message" : "Hello, World!"
    });
    Json(json_response)
}