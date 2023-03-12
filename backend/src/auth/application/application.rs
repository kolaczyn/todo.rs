use sqlx::PgPool;

use crate::{
    auth::{
        domain::validation::are_credentials_valid,
        repository::repository::{login_db, register_db},
    },
    common::jwt::create_jwt,
};

use super::{dto::UserDto, error::ErrorAuth};

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
    let jwt = create_jwt(user_db.id, &user_db.email).map_err(|_| ErrorAuth::JwtCreation)?;

    Ok(UserDto {
        email: user_db.email.clone(),
        id: user_db.id,
        jwt,
    })
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
    let jwt = create_jwt(user_db.id, &user_db.email).map_err(|_| ErrorAuth::JwtCreation)?;

    Ok(UserDto {
        email: user_db.email.clone(),
        id: user_db.id,
        jwt,
    })
}
