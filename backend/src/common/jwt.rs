use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub id: i32,
    pub exp: usize,
}
