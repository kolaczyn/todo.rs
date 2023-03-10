use tide::Request;

use crate::state::State;

use super::{
    dto::{CreateTodoDto, TodoDto, UpdateTodoDto},
    repository::{create_todo_db, delete_todo_db, get_todo_db, get_todos_db, update_todo_db},
};

async fn get_todos(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let todos = get_todos_db(&pool).await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn get_todo(req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let pool = req.state().pool.clone();

    match get_todo_db(&pool, id).await {
        Ok(todo) => Ok(serde_json::to_string_pretty(&todo)?),
        Err(_) => Err(tide::Error::from_str(404, "Todo not found")),
    }
}

async fn create_todo(mut req: Request<State>) -> tide::Result<String> {
    let label = req.body_json::<CreateTodoDto>().await?.label;
    let pool = req.state().pool.clone();

    let todo = create_todo_db(&pool, label).await?;
    Ok(serde_json::to_string_pretty(&todo)?)
}

async fn update_todo(mut req: Request<State>) -> tide::Result<String> {
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

async fn delete_todo(req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let pool = req.state().pool.clone();

    let todo_db = delete_todo_db(&pool, id).await?;

    Ok(serde_json::to_string_pretty(&todo_db)?)
}

pub fn todo_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);
    api.at("/").get(get_todos);
    api.at("/:id").get(get_todo);
    api.at("/").post(create_todo);
    api.at("/:id").patch(update_todo);
    api.at("/:id").delete(delete_todo);
    api
}
