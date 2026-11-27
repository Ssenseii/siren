use std::sync::Arc;

use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

mod handlers;
mod model;
mod schema;
mod route;

use handlers::hello_world;
use crate::route::create_router;

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
    let app = create_router(Arc::new(AppState {db: pool.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

