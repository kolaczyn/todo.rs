use sqlx::PgPool;
use thiserror::Error;

use crate::todos::repository::{
    models::{TodoWithCategoryDb, TodoWithoutCategoryDb},
    repository::{
        assign_todo_to_category_db, create_todo_db, delete_todo_db, get_todo_db, get_todos_db,
        update_todo_db,
    },
};

#[derive(Error, Debug)]
pub enum ErrorTodos {
    #[error("Not found")]
    NotFound,
    #[error("Db error")]
    DbError,
}

fn db_err_to_app_err(err: sqlx::Error) -> ErrorTodos {
    match err {
        sqlx::Error::ColumnNotFound(_) => ErrorTodos::NotFound,
        _ => ErrorTodos::DbError,
    }
}

// TODO return TodoWithCategoryDto instead of TodoWithCategoryDb
pub async fn get_todos_app(pool: &PgPool) -> Result<Vec<TodoWithCategoryDb>, ErrorTodos> {
    let todos = get_todos_db(&pool).await.map_err(db_err_to_app_err);
    todos
}

pub async fn get_todo_app(pool: &PgPool, id: i32) -> Result<TodoWithoutCategoryDb, ErrorTodos> {
    let todo = get_todo_db(pool, id).await.map_err(db_err_to_app_err);
    todo
}

pub async fn create_todo_app(
    pool: &PgPool,
    label: &String,
) -> Result<TodoWithoutCategoryDb, ErrorTodos> {
    let todo = create_todo_db(pool, label).await.map_err(db_err_to_app_err);
    todo
}

pub async fn update_todo_app(
    pool: &PgPool,
    id: i32,
    completed: bool,
) -> Result<TodoWithoutCategoryDb, ErrorTodos> {
    let todo = update_todo_db(pool, id, completed)
        .await
        .map_err(db_err_to_app_err);
    todo
}

pub async fn assign_todo_to_category_app(
    pool: &PgPool,
    todo_id: i32,
    category_id: i32,
) -> Result<TodoWithoutCategoryDb, ErrorTodos> {
    let todo = assign_todo_to_category_db(pool, todo_id, category_id)
        .await
        .map_err(db_err_to_app_err);
    todo
}

pub async fn delete_todo_app(pool: &PgPool, id: i32) -> Result<TodoWithoutCategoryDb, ErrorTodos> {
    let todo = delete_todo_db(pool, id).await.map_err(db_err_to_app_err);
    todo
}
