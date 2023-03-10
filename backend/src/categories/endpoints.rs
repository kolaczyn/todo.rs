use tide::Request;

use crate::state::State;

use super::{
    dto::CreateCategoryDto,
    repository::{create_category_db, get_categories_db},
};

async fn get_categories(req: Request<State>) -> tide::Result<String> {
    let pool = &req.state().pool;
    let categories = get_categories_db(pool).await?;

    Ok(serde_json::to_string_pretty(&categories)?)
}

async fn create_category(mut req: Request<State>) -> tide::Result<String> {
    let body: CreateCategoryDto = req.body_json().await?;
    let pool = &req.state().pool;

    let label = body.label;
    let color = body.color;

    let categories = create_category_db(pool, label, color).await?;

    Ok(serde_json::to_string_pretty(&categories)?)
}

pub fn categories_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);
    api.at("/").get(get_categories);
    api.at("/").post(create_category);
    api
}
