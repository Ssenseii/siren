use serde::{Deserialize, Serialize};
use uuid::Uuid;

// deserialize: json to struct
// serialize: struct to json
// fromrow: db row to struct
/* Without this derive macro, we’d have to manually write a lot of lines of code to get all the above-mentioned functionality. */

// internam
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>, // option cause created_at is optional
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,  
} 

// public
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    
    #[serde(skip_serializing)]
    pub password: String,

    pub created_at: Option<chrono::DateTime<chrono::Utc>>, // option cause created_at is optional
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,  
} 