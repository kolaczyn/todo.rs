use crate::categories::repository::models::CategoryDb;

use super::dto::CategoryDto;

impl CategoryDb {
    pub fn to_dto(&self) -> CategoryDto {
        CategoryDto {
            id: self.id,
            label: String::from(&self.label),
            color: String::from(&self.color),
        }
    }
}
