use sqlx::PgPool;
use thiserror::Error;

use crate::todos::repository::{
    models::TodoDb,
    repository::{
        assign_todo_to_category_db, create_todo_db, delete_todo_db, get_todo_db, get_todos_db,
        update_todo_db,
    },
};

use super::dto::TodoDto;

#[derive(Error, Debug)]
pub enum ErrorTodos {
    #[error("Not found")]
    NotFound,
    #[error("Db error")]
    DbError,
    #[error("Unauthorized")]
    Unauthorized,
}

fn db_err_to_app_err(err: sqlx::Error) -> ErrorTodos {
    match err {
        sqlx::Error::ColumnNotFound(_) => ErrorTodos::NotFound,
        _ => ErrorTodos::DbError,
    }
}

// TOOD this should return something like Result<bool, DbError>
async fn can_user_access_todo(pool: &PgPool, todo_id: i32, user_id: i32) -> bool {
    let todo = get_todo_db(pool, todo_id).await;
    match todo {
        Ok(todo) => todo.user_id.contains(&user_id),
        Err(_) => false,
    }
}

pub async fn get_todos_app(pool: &PgPool, user_id: i32) -> Result<Vec<TodoDto>, ErrorTodos> {
    let todos_db = get_todos_db(&pool, user_id)
        .await
        .map_err(db_err_to_app_err)?;
    let todos_dto = todos_db.into_iter().map(|x| x.to_dto()).collect();
    Ok(todos_dto)
}

pub async fn get_todo_app(
    pool: &PgPool,
    todo_id: i32,
    user_id: i32,
) -> Result<TodoDto, ErrorTodos> {
    let todo = get_todo_db(pool, todo_id)
        .await
        .map_err(db_err_to_app_err)?;

    if todo.user_id.contains(&user_id) {
        return Ok(todo.to_dto());
    }
    return Err(ErrorTodos::NotFound);
}

pub async fn create_todo_app(
    pool: &PgPool,
    label: &String,
    user_id: i32,
) -> Result<TodoDto, ErrorTodos> {
    let todo = create_todo_db(pool, label, user_id)
        .await
        .map_err(db_err_to_app_err)?;
    Ok(todo.to_dto())
}

pub async fn update_todo_app(
    pool: &PgPool,
    todo_id: i32,
    completed: bool,
    user_id: i32,
) -> Result<TodoDto, ErrorTodos> {
    let can_access = can_user_access_todo(pool, todo_id, user_id).await;
    if !can_access {
        return Err(ErrorTodos::Unauthorized);
    }
    let todo = update_todo_db(pool, todo_id, completed)
        .await
        .map_err(db_err_to_app_err)?;
    Ok(todo.to_dto())
}

pub async fn assign_todo_to_category_app(
    pool: &PgPool,
    todo_id: i32,
    category_id: i32,
    user_id: i32,
) -> Result<TodoDto, ErrorTodos> {
    let can_access = can_user_access_todo(pool, todo_id, user_id).await;
    if !can_access {
        return Err(ErrorTodos::Unauthorized);
    }

    let todo = assign_todo_to_category_db(pool, todo_id, category_id)
        .await
        .map_err(db_err_to_app_err)?;
    Ok(todo.to_dto())
}

pub async fn delete_todo_app(
    pool: &PgPool,
    todo_id: i32,
    user_id: i32,
) -> Result<TodoDb, ErrorTodos> {
    let can_access = can_user_access_todo(pool, todo_id, user_id).await;
    if !can_access {
        return Err(ErrorTodos::Unauthorized);
    }

    let todo = delete_todo_db(pool, todo_id)
        .await
        .map_err(db_err_to_app_err);
    todo
}
