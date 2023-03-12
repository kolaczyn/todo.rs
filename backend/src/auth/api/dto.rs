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

#[derive(Deserialize)]
pub struct MeFormDto {
    pub jwt: String,
}

#[derive(Serialize)]
pub struct UserDto {
    pub id: i32,
    pub email: String,
    pub jwt: String,
}
