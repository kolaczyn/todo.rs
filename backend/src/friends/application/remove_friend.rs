use sqlx::PgPool;

use crate::friends::repository::remove_friend::remove_friend_db;

pub async fn remove_friend_app(
    pool: &PgPool,
    user_id: i32,
    friend_id: i32,
) -> Result<(), sqlx::Error> {
    remove_friend_db(pool, user_id, friend_id).await?;
    Ok(())
}
