use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDb {
    pub id: i32,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginQueryDb {
    pub email: String,
    pub password_hash: String,
    pub id: i32,
}
