use tide::Request;

use crate::state::State;

use super::{
    dto::{CreateTodoDto, TodoDto, UpdateTodoDto},
    repository::{create_todo_db, delete_todo_db, get_todo_db, get_todos_db, update_todo_db},
};

pub async fn get_todos_endpoint(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let todos = get_todos_db(&pool).await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

pub async fn get_todo_endpoint(req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let pool = req.state().pool.clone();

    let todos = get_todo_db(&pool, id).await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

pub async fn create_todo_endpoint(mut req: Request<State>) -> tide::Result<String> {
    let label = req.body_json::<CreateTodoDto>().await?.label;
    let pool = req.state().pool.clone();

    let todo = create_todo_db(&pool, label).await?;
    Ok(serde_json::to_string_pretty(&todo)?)
}

pub async fn update_todo_endpoint(mut req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let completed = req.body_json::<UpdateTodoDto>().await?.completed;
    let pool = req.state().pool.clone();

    // yeah, the names here are pretty confusing :p
    let todo_db = get_todo_db(&pool, id).await?;
    let new_todo = TodoDto {
        completed,
        ..todo_db
    };
    let new_todo_db = update_todo_db(&pool, new_todo).await?;

    Ok(serde_json::to_string_pretty(&new_todo_db)?)
}

pub async fn delete_todo_endpoint(req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let pool = req.state().pool.clone();

    let todo_db = delete_todo_db(&pool, id).await?;

    Ok(serde_json::to_string_pretty(&todo_db)?)
}

pub fn todo_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);
    api.at("/").get(get_todos_endpoint);
    api.at("/:id").get(get_todo_endpoint);
    api.at("/").post(create_todo_endpoint);
    api.at("/:id").patch(update_todo_endpoint);
    api.at("/:id").delete(delete_todo_endpoint);
    api
}
