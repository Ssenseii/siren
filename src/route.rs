use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{AppState, handlers::{create_user_handler, login_user_handler, user_list_handler, get_user_handler}};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/user", post(create_user_handler))
    .route("/api/users", get(user_list_handler))
    .route("/api/user/{id}", get(get_user_handler))
    .route("/api/login", post(login_user_handler))
    .with_state(app_state)
}