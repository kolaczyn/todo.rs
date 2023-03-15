use tide::Request;

use crate::{
    categories::{
        application::dto::CategoryDto,
        repository::repository::{create_category_db, get_categories_db, get_category_db},
    },
    common::jwt::Claims,
    state::State,
};

use super::form::CreateCategoryDto;

async fn get_categories(req: Request<State>) -> tide::Result<String> {
    let _claims = req.ext::<Claims>();
    let pool = &req.state().pool;

    let categories: Vec<CategoryDto> = get_categories_db(pool)
        .await?
        .into_iter()
        .map(|x| x.to_dto())
        .collect();

    Ok(serde_json::to_string_pretty(&categories)?)
}

async fn get_category(req: Request<State>) -> tide::Result<String> {
    let _claims = req.ext::<Claims>();
    let pool = &req.state().pool;
    let id: i32 = req.param("id")?.parse()?;

    let category: CategoryDto = get_category_db(pool, id).await?.to_dto();

    Ok(serde_json::to_string_pretty(&category)?)
}

async fn create_category(mut req: Request<State>) -> tide::Result<String> {
    let _claims = req.ext::<Claims>();
    let body: CreateCategoryDto = req.body_json().await?;
    let pool = &req.state().pool;

    let categories: Vec<CategoryDto> = create_category_db(&pool, &body.label, &body.color)
        .await?
        .into_iter()
        .map(|x| x.to_dto())
        .collect();

    Ok(serde_json::to_string_pretty(&categories)?)
}

pub fn categories_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/").get(get_categories);
    api.at("/:id").get(get_category);
    api.at("/").post(create_category);

    api
}
