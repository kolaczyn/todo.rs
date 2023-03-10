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
    pub category: Option<TodoCategoryDto>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoDto {
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoDto {
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoCategoryDto {
    pub category_id: i32,
}
