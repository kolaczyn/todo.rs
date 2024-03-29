use sqlx::PgPool;

use crate::{
    auth::repository::repository::{login_db, register_db},
    common::get_jwt::get_jwt,
};

use super::{dto::UserDto, error::ErrorAuth, validation::are_credentials_valid};

pub async fn register_app(
    pool: &PgPool,
    email: &String,
    password: &String,
) -> Result<UserDto, ErrorAuth> {
    if !are_credentials_valid(&email, &password) {
        return Err(ErrorAuth::Validation);
    }
    let user_db = register_db(&pool, &email, &password)
        .await
        .map_err(|_| ErrorAuth::InvalidCredentials)?;

    let jwt = get_jwt(&user_db.email, user_db.id).map_err(|_| ErrorAuth::JwtCreation)?;
    Ok(user_db.to_dto(jwt))
}

pub async fn login_app(
    pool: &PgPool,
    email: &String,
    password: &String,
) -> Result<UserDto, ErrorAuth> {
    if !are_credentials_valid(&email, &password) {
        return Err(ErrorAuth::Validation);
    }
    let user_db = login_db(&pool, &email, &password)
        .await
        .map_err(|_| ErrorAuth::InvalidCredentials)?;

    let jwt = get_jwt(&user_db.email, user_db.id).map_err(|_| ErrorAuth::JwtCreation)?;
    Ok(user_db.to_dto(jwt))
}
