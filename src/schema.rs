use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSchema {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserSchema {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSchema {
    pub username: String,
    pub password: String,
}


/*
{
    "username": "user.lastname",
    "email": "user@example.com",
    "password": "AISux@123"
}

*/