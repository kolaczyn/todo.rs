use sqlx::PgPool;

use crate::friends::repository::get_friends::get_friends_db;

use super::dto::FriendDto;

pub async fn get_friends_app(pool: &PgPool, user_id: i32) -> Result<Vec<FriendDto>, sqlx::Error> {
    let friends = get_friends_db(pool, user_id).await?;
    let friends = friends
        .into_iter()
        .map(|friend| FriendDto {
            user_id: friend.user_id,
        })
        .collect();
    Ok(friends)
}
