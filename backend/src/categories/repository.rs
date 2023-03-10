use anyhow::Error;
use sqlx::PgPool;

use super::dto::CategoryDto;

pub async fn get_categories(pool: &PgPool) -> Result<Vec<CategoryDto>, Error> {
    Ok(vec![])
    // let categories_db = sqxl::query!("SELECT id, label, color FROM categories");
    // Ok(categories_db)
}
