use std::sync::Arc;

use axum::{Router, routing::get};
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

mod handlers;
use handlers::hello_world;

pub struct AppState {
    db: PgPool,
}

// this allows the main to be async.
#[tokio::main]
async fn main(){ 

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABAS_URL must be configured");
    
    let pool = match PgPoolOptions::new().max_connections(10).connect(&db_url).await{
        Ok(pool) => { println!("Connected to DB successfully"); pool }
        Err(err) => { println!("Failed to connect to DB: {}", err); std::process::exit(1) }
    };

    // server
    let app = Router::new().route("/api", get(hello_world)).with_state(Arc::new(AppState {db: pool.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

