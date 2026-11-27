use std::{sync::Arc};

use axum::{Json, extract::{State, Path}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use dotenv::dotenv;
use uuid::Uuid;

use crate::{AppState, model::{User, UserResponse}, schema::{LoginSchema, UserSchema}};

use argon2::{
    password_hash::{PasswordHasher, phc::PasswordHash, PasswordVerifier}, 
    Algorithm, Argon2, Params, Version
};


/// TODO
/// Rate Limitting
/// JWT Access Token
/// Refresh tokens

pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    dotenv().ok();
    let id = uuid::Uuid::new_v4();

    // hashing
    let password = body.password.as_bytes();
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


pub async fn user_list_handler(
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let users = sqlx::query_as!(
        UserResponse,
        r#"SELECT * FROM users ORDER by created_at"#
    ).fetch_all(&data.db)
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": e.to_string()})),
    ))?;

    // Filter Password
    
    let json_response = serde_json::json!({
        "status" : "ok",
        "count" : users.len(),
        "data": users
    });

    Ok(Json(json_response))
}



pub async fn get_user_handler(
    Path(user_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user = sqlx::query_as!(
        UserResponse,
        r#"SELECT * FROM users WHERE id = $1"#,
        &user_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": e.to_string()})),
    ))?;

    let user_response = json!({
        "status": "success",
        "data": json!({
                "user": user
                })
            });

    Ok(Json(user_response))
}

pub async fn delete_user_handler(
    Path(user_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let query_result = sqlx::query_as!(
        UserResponse,
        r#"DELETE FROM users WHERE id = $1 RETURNING *"#,
        &user_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "User not found"
            })),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        ),
    })?;

    let response = json!({
        "status": "success",
        "message": "Game delete successfully",
        "data": {
            "deleted_user": query_result
        }
    });

    Ok(Json(response))
}





pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginSchema>) 
    -> 
    Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{

        // get the user from the DB
    let user = sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password, created_at, updated_at
                FROM users
                WHERE username = $1 
                LIMIT 1
            "#,
            body.username,
        )
        .fetch_optional(&data.db)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
    ))?;

    let user = user.ok_or((
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Invalid credentials"})),
    ))?;

    // Verify password
    let password = body.password.as_bytes();
    let pepper = std::env::var("PEPPER_ARGON").map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Auth Configuration Error"})),
        ))?;
    let pepper_as_bytes = pepper.as_bytes();

    let parsed_hash = PasswordHash::new(&user.password).map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Invalid stored password hash"})),
        ))?;
    
    let argon2 = Argon2::new_with_secret(
        pepper_as_bytes,
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Authentication configuration error"})),
        ))?;

    argon2.verify_password(password, &parsed_hash) .map_err(|_| (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ))?;

    
    let login_response = json!({
        "status": "success",
        "data": {
            "user": {
                "id": user.id,
                "username": user.username,
                "email": user.email,
            }
        }
    });

    Ok(Json(login_response))
}





// for history
pub async fn hello_world() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = json!({
        "status" : "ok",
        "message" : "Hello, World!"
    });
    
    Ok(Json(json_response))
}


