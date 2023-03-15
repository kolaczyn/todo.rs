use anyhow::{anyhow, Error, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;

use crate::auth::repository::models::LoginQueryDb;

use super::models::UserDb;

pub async fn register_db(
    pool: &PgPool,
    email: &String,
    password: &String,
) -> Result<UserDb, Error> {
    let hashed_password = hash(password, DEFAULT_COST)?;

    let user = sqlx::query_as!(
        UserDb,
        "
        INSERT INTO users(email, password_hash)
        VALUES($1, $2)
        RETURNING id, email
        ",
        email,
        hashed_password
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn login_db(pool: &PgPool, email: &String, password: &String) -> Result<UserDb> {
    let user = sqlx::query_as!(
        LoginQueryDb,
        "
        SELECT email, password_hash, id
        FROM users
        WHERE email = $1
        ",
        email
    )
    .fetch_one(pool)
    .await?;

    let is_password_ok = verify(password, &user.password_hash)?;
    match is_password_ok {
        true => Ok(UserDb {
            email: user.email,
            id: user.id,
        }),
        false => Err(anyhow!(sqlx::Error::RowNotFound)),
    }
}
