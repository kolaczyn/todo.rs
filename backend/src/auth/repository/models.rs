use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDb {
    pub id: i32,
    pub email: String,
}
