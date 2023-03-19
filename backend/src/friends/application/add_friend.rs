use sqlx::PgPool;

use crate::friends::repository::add_friend::add_friend_db;

pub async fn add_friend_app(
    pool: &PgPool,
    user_id: i32,
    friend_id: i32,
) -> Result<(), sqlx::Error> {
    add_friend_db(pool, user_id, friend_id).await?;
    Ok(())
}
