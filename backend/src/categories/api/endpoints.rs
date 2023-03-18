use tide::Request;

use crate::{
    categories::application::applicaiton::{
        create_category_app, get_categories_app, get_category_app,
    },
    common::{http_error::HttpError, jwt::Claims},
    state::State,
};

use super::form::CreateCategoryDto;

async fn get_categories(req: Request<State>) -> tide::Result<String> {
    let _claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let pool = &req.state().pool;

    let categories = get_categories_app(pool).await?;

    Ok(serde_json::to_string_pretty(&categories)?)
}

async fn get_category(req: Request<State>) -> tide::Result<String> {
    let _claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let pool = &req.state().pool;
    let id: i32 = req.param("id")?.parse()?;

    let category = get_category_app(pool, id).await?;

    Ok(serde_json::to_string_pretty(&category)?)
}

async fn create_category(mut req: Request<State>) -> tide::Result<String> {
    let _claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let body: CreateCategoryDto = req.body_json().await?;
    let pool = &req.state().pool;

    let categories = create_category_app(&pool, &body.label, &body.color).await?;

    Ok(serde_json::to_string_pretty(&categories)?)
}

pub fn categories_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/").get(get_categories);
    api.at("/:id").get(get_category);
    api.at("/").post(create_category);

    api
}
