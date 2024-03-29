use tide::Request;

use crate::{
    common::{http_error::HttpError, jwt::Claims},
    state::State,
    todos::application::application::{
        assign_todo_to_category_app, create_todo_app, delete_todo_app, get_todo_app, get_todos_app,
        update_todo_app, ErrorTodos,
    },
};

use super::form::{CreateTodoForm, UpdateTodoCategoryForm, UpdateTodoForm};

fn app_err_to_response(err: ErrorTodos) -> tide::Error {
    match err {
        ErrorTodos::NotFound => tide::Error::from_str(404, "Not found"),
        ErrorTodos::DbError => tide::Error::from_str(500, "Server eror"),
        ErrorTodos::Unauthorized => tide::Error::from_str(401, "Unauthorized"),
    }
}

async fn get_todos(req: Request<State>) -> tide::Result<String> {
    let claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let pool = &req.state().pool;

    let todos = get_todos_app(pool, claims.id).await;
    match todos.map_err(app_err_to_response) {
        Ok(todos) => Ok(serde_json::to_string_pretty(&todos)?),
        Err(err) => Err(err),
    }
}

async fn get_todo(req: Request<State>) -> tide::Result<String> {
    let claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let pool = &req.state().pool;
    let todo_id: i32 = req.param("id")?.parse()?;

    let todo = get_todo_app(pool, todo_id, claims.id).await;
    match todo.map_err(app_err_to_response) {
        Ok(todo) => Ok(serde_json::to_string_pretty(&todo)?),
        Err(err) => Err(err),
    }
}

async fn create_todo(mut req: Request<State>) -> tide::Result<String> {
    let label = req.body_json::<CreateTodoForm>().await?.label;
    let claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let pool = &req.state().pool;

    let todo = create_todo_app(&pool, &label, claims.id).await;
    match todo.map_err(app_err_to_response) {
        Ok(todo) => Ok(serde_json::to_string_pretty(&todo)?),
        Err(err) => Err(err),
    }
}

async fn update_todo(mut req: Request<State>) -> tide::Result<String> {
    let completed = req.body_json::<UpdateTodoForm>().await?.completed;
    let claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let pool = &req.state().pool;
    let todo_id: i32 = req.param("id")?.parse()?;

    let todo = update_todo_app(&pool, todo_id, completed, claims.id).await;
    match todo.map_err(app_err_to_response) {
        Ok(todo) => Ok(serde_json::to_string_pretty(&todo)?),
        Err(err) => Err(err),
    }
}

async fn assign_todo_to_category(mut req: Request<State>) -> tide::Result<String> {
    let category_id = req.body_json::<UpdateTodoCategoryForm>().await?.category_id;
    let claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let pool = &req.state().pool;
    let todo_id: i32 = req.param("id")?.parse()?;

    let todo = assign_todo_to_category_app(pool, todo_id, category_id, claims.id).await;
    match todo.map_err(app_err_to_response) {
        Ok(todo) => Ok(serde_json::to_string_pretty(&todo)?),
        Err(err) => Err(err),
    }
}

async fn delete_todo(req: Request<State>) -> tide::Result<String> {
    let claims = req.ext::<Claims>().ok_or(HttpError::Unauthorized)?;
    let pool = &req.state().pool;
    let todo_id: i32 = req.param("id")?.parse()?;

    let todo = delete_todo_app(pool, todo_id, claims.id).await;
    match todo.map_err(app_err_to_response) {
        Ok(todo) => Ok(serde_json::to_string_pretty(&todo)?),
        Err(err) => Err(err),
    }
}

pub fn todo_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/").get(get_todos);
    api.at("/:id").get(get_todo);
    api.at("/").post(create_todo);
    // TODO "merge" the two patch endpoints
    api.at("/:id").patch(update_todo);
    api.at("/assign-to-category/:id")
        .patch(assign_todo_to_category);
    api.at("/:id").delete(delete_todo);

    api
}
