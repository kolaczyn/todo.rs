use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct CategoryDto {
    pub id: i32,
    pub label: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub label: String,
    pub color: String,
}
