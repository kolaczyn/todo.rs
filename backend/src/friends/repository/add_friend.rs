use sqlx::PgPool;

use super::models::FriendDb;

pub async fn add_friend_db(
    pool: &PgPool,
    user_id: i32,
    friend_id: i32,
) -> Result<FriendDb, sqlx::Error> {
    sqlx::query_as!(
        FriendDb,
        r#"
        INSERT INTO friends (user_id, friend_id)
        VALUES ($1, $2)
        RETURNING id, user_id, friend_id
        "#,
        user_id,
        friend_id
    )
    .fetch_one(pool)
    .await
}
