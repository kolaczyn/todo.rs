use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub label: String,
    pub color: String,
}
