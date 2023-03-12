use serde::{Deserialize, Serialize};

use crate::todos::api::dto::TodoDto;

#[derive(Serialize, Deserialize)]
pub struct TodoCategoryDb {
    pub id: i32,
    pub label: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoDb {
    pub id: i32,
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
}

impl TodoDb {
    pub fn to_dto(&self) -> TodoDto {
        TodoDto {
            id: self.id,
            label: self.label.to_owned(),
            description: self.description.to_owned(),
            completed: self.completed,
            // TODO fix
            category: None,
        }
    }
}
