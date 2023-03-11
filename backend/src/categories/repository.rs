use anyhow::Error;
use sqlx::PgPool;

use crate::categories::db_dto::CategoryDb;

use super::dto::CategoryDto;

pub async fn get_categories_db(pool: &PgPool) -> Result<Vec<CategoryDto>, Error> {
    let categories = sqlx::query_as!(CategoryDb, "SELECT id, label, color FROM categories")
        .fetch_all(pool)
        .await?
        .iter()
        .map(|x| x.to_dto())
        .collect();

    Ok(categories)
}

pub async fn get_category_db(pool: &PgPool, id: i32) -> Result<CategoryDto, Error> {
    let category = sqlx::query!("SELECT id, label, color FROM categories WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(CategoryDto {
        id: category.id,
        label: category.label.to_owned(),
        color: category.color.to_owned(),
    })
}

pub async fn create_category_db(
    pool: &PgPool,
    label: String,
    color: String,
) -> Result<Vec<CategoryDto>, Error> {
    let categories = sqlx::query_as!(
        CategoryDb,
        "INSERT INTO categories(label, color)
        VALUES($1, $2)
        RETURNING label, color, id",
        label,
        color
    )
    .fetch_all(pool)
    .await?
    .iter()
    .map(|x| x.to_dto())
    .collect();

    Ok(categories)
}
