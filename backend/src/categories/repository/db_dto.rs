use crate::categories::api::dto::CategoryDto;

pub struct CategoryDb {
    pub id: i32,
    pub label: String,
    pub color: String,
}

impl CategoryDb {
    pub fn to_dto(&self) -> CategoryDto {
        CategoryDto {
            id: self.id,
            label: self.label.to_string(),
            color: self.color.to_string(),
        }
    }
}
