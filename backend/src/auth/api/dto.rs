use serde::Deserialize;

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
