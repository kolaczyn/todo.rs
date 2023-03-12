use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterFormDto {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginFormDto {
    pub email: String,
    pub password: String,
}

pub struct UserDto {
    pub email: String,
    pub password: String,
}
