use std::env;

use dotenv::dotenv;

use crate::{
    state::State,
    todos::endpoints::{
        create_todo_endpoint, delete_todo_endpoint, get_todo_endpoint, get_todos_endpoint,
        update_todo_endpoint,
    },
};
mod state;
mod todo;
mod todos;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv()?;
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let state = State::new(database_url).await?;
    let mut app = tide::with_state(state);

    tide::log::start();

    app.at("/todos").get(get_todos_endpoint);
    app.at("/todos/:id").get(get_todo_endpoint);
    app.at("/todos").post(create_todo_endpoint);
    app.at("/todos/:id").patch(update_todo_endpoint);
    app.at("/todos/:id").delete(delete_todo_endpoint);

    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}
