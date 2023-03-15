use crate::categories::repository::models::CategoryDb;

use super::dto::CategoryDto;

impl CategoryDb {
    pub fn to_dto(self) -> CategoryDto {
        CategoryDto {
            id: self.id,
            label: self.label,
            color: self.color,
        }
    }
}
