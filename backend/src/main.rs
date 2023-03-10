use std::env;

use dotenv::dotenv;

use crate::{
    categories::endpoints::categories_endpoints, state::State, todos::endpoints::todo_endpoints,
};
mod categories;
mod state;
mod todos;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv()?;
    let port = env::var("PORT").unwrap_or(String::from("8080"));

    let state = State::new().await?;
    let mut app = tide::new();

    tide::log::start();

    app.at("/todos").nest(todo_endpoints(state.clone()));
    app.at("/categories").nest(categories_endpoints(state));

    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}
