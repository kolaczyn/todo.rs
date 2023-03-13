use crate::todos::repository::models::TodoWithCategoryDb;

use super::dto::{TodoCategoryDto, TodoDto};

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
