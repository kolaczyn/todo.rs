use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoDb {
    pub id: i32,
    pub label: String,
    pub completed: bool,
    pub description: Option<String>,
    pub category_id: Option<i32>,
}
