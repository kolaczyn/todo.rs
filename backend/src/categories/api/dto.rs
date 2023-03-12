use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
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
