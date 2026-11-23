use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSchema {
    pub username: String,
    pub email: String,
    pub password: String,
}

/*
{
    "username": "user.lastname",
    "email": "user@example.com",
    "password": "AISux@123"
}

*/