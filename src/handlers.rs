use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use dotenv::dotenv;

use crate::{AppState, model::User, schema::UserSchema};

use argon2::{
    password_hash::{PasswordHasher},
    Algorithm, Argon2, Params, Version
};


pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    dotenv().ok();
    let id = uuid::Uuid::new_v4();

    // hashing
    let password = b"body.password";
    let pepper = std::env::var("PEPPER_ARGON").expect("PEPPER_ARGON Needs to be configured");
    let pepper_as_bytes = pepper.as_bytes();

    let argon2 = Argon2::new_with_secret(
    pepper_as_bytes,
    Algorithm::default(),
    Version::default(),
    Params::default()
    ).unwrap();

    let password_hash = argon2.hash_password(password).unwrap().to_string();

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
            password_hash
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