use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{AppState, handlers::{create_user_handler, login_user_handler}};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/user", post(create_user_handler))
    .route("/api/login", post(login_user_handler))
    .with_state(app_state)
}