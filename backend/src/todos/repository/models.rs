use serde::{Deserialize, Serialize};

use crate::todos::api::form::TodoDto;

#[derive(Serialize, Deserialize)]
pub struct TodoWithCategoryDb {
    pub id: i32,
    pub label: String,
    pub completed: bool,
    pub description: Option<String>,

    pub category_id: Option<i32>,
    pub category_label: Option<String>,
    pub category_color: Option<String>,
}

//  TODO this should be removed and replaced with TodoWithCategoryDb
#[derive(Serialize, Deserialize)]
pub struct TodoWithoutCategoryDb {
    pub id: i32,
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
}

impl TodoWithoutCategoryDb {
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
