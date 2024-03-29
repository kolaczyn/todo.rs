use crate::todos::repository::models::TodoDb;

use super::dto::TodoDto;

impl TodoDb {
    pub fn to_dto(self) -> TodoDto {
        TodoDto {
            id: self.id,
            completed: self.completed,
            label: self.label,
            description: self.description,
            category_id: self.category_id,
        }
    }
}
