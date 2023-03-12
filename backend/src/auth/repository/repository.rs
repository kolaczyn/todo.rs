use sqlx::PgPool;

use super::models::UserDb;

pub async fn register_db(
    _pool: &PgPool,
    username: String,
    password: String,
) -> Result<UserDb, sqlx::Error> {
    let user = UserDb {
        id: 1,
        username,
        password,
    };

    Ok(user)
}

pub async fn login_db(
    _pool: &PgPool,
    username: String,
    password: String,
) -> Result<UserDb, sqlx::Error> {
    let user = UserDb {
        id: 1,
        username,
        password,
    };

    Ok(user)
}
