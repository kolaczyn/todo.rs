use sqlx::PgPool;

use crate::categories::repository::repository::{
    create_category_db, get_categories_db, get_category_db,
};

use super::dto::CategoryDto;

pub async fn get_categories_app(pool: &PgPool) -> Result<Vec<CategoryDto>, sqlx::Error> {
    let categories: Vec<CategoryDto> = get_categories_db(pool)
        .await?
        .into_iter()
        .map(|x| x.to_dto())
        .collect();

    Ok(categories)
}

pub async fn get_category_app(pool: &PgPool, id: i32) -> Result<CategoryDto, sqlx::Error> {
    let category: CategoryDto = get_category_db(pool, id).await?.to_dto();

    Ok(category)
}

pub async fn create_category_app(
    pool: &PgPool,
    label: &String,
    color: &String,
) -> Result<Vec<CategoryDto>, sqlx::Error> {
    let categories: Vec<CategoryDto> = create_category_db(&pool, &label, &color)
        .await?
        .into_iter()
        .map(|x| x.to_dto())
        .collect();

    Ok(categories)
}
