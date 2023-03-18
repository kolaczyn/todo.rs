use sqlx::PgPool;
use thiserror::Error;

use crate::categories::repository::repository::{
    create_category_db, get_categories_db, get_category_db,
};

#[derive(Error, Debug)]
pub enum ErrorCategories {
    #[error("Not found")]
    NotFound,
    #[error("Db error")]
    _DbError,
    #[error("Unauthorized")]
    _Unauthorized,
    #[error("Color is not nice")]
    ColorIsNotNice,
}

use super::{color_validation::is_color_nice, dto::CategoryDto};

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
) -> Result<Vec<CategoryDto>, ErrorCategories> {
    if !is_color_nice(color) {
        return Err(ErrorCategories::ColorIsNotNice);
    }
    let categories: Vec<CategoryDto> = create_category_db(&pool, &label, &color)
        .await
        .map_err(|_| ErrorCategories::NotFound)?
        .into_iter()
        .map(|x| x.to_dto())
        .collect();

    Ok(categories)
}
