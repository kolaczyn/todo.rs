use sqlx::PgPool;

use crate::friends::repository::models::FriendDb;

pub async fn get_friends_db(pool: &PgPool, user_id: i32) -> Result<Vec<FriendDb>, sqlx::Error> {
    sqlx::query_as!(
        FriendDb,
        r#"
        SELECT
            f.id,
            f.user_id,
            f.friend_id
        FROM friends f
        WHERE f.user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}
