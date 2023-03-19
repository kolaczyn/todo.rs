use sqlx::PgPool;

use crate::friends::repository::models::FriendDb;

pub async fn remove_friend_db(
    pool: &PgPool,
    user_id: i32,
    friend_id: i32,
) -> Result<FriendDb, sqlx::Error> {
    sqlx::query_as!(
        FriendDb,
        "
        DELETE FROM friends
        WHERE user_id = $1 AND friend_id = $2
        RETURNING id, user_id, friend_id
        ",
        user_id,
        friend_id
    )
    .fetch_one(pool)
    .await
}
