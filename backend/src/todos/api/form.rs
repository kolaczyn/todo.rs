use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTodoForm {
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoForm {
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoCategoryForm {
    pub category_id: Option<i32>,
}
