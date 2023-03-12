use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDb {
    pub id: i32,
    pub username: String,
    // TODO will have to replace it with hash
    pub password: String,
}
