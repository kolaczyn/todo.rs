use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoCategoryDto {
    pub id: i32,
    pub label: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoDto {
    pub id: i32,
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
    pub category_id: Option<i32>,
}
