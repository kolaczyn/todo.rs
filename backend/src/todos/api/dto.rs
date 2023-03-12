use serde::{Deserialize, Serialize};

use crate::todos::repository::db_dto::TodoWithCategoryDb;

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

impl TodoWithCategoryDb {
    fn get_category(&self) -> Option<TodoCategoryDto> {
        match (
            self.category_id,
            self.category_label.to_owned(),
            self.category_color.to_owned(),
        ) {
            (Some(c_id), Some(c_label), Some(c_color)) => Some(TodoCategoryDto {
                id: c_id,
                label: c_label,
                color: c_color,
            }),
            _ => None,
        }
    }
    pub fn to_dto(&self) -> TodoDto {
        TodoDto {
            id: self.id,
            completed: self.completed,
            label: self.label.to_owned(),
            description: self.description.to_owned(),
            category: self.get_category(),
        }
    }
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
