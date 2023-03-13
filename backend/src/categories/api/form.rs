use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub label: String,
    pub color: String,
}
