use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoDb {
    pub id: i32,
    // TODO drop all Todos without user or category_id and make the fields not null
    pub user_id: Option<i32>,
    pub label: String,
    pub completed: bool,
    pub description: Option<String>,
    pub category_id: Option<i32>,
}
