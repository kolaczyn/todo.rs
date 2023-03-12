use anyhow::Error;
use sqlx::PgPool;

use crate::todos::{
    api::dto::TodoDto,
    repository::db_dto::{TodoWithCategoryDb, TodoWithoutCategoryDb},
};

pub async fn get_todos_db(pool: &PgPool) -> Result<Vec<TodoWithCategoryDb>, sqlx::Error> {
    let todos = sqlx::query_as!(
        TodoWithCategoryDb,
        "
        SELECT completed, t.label AS label, t.id AS id, description,
        c.id AS category_id, c.label AS category_label, c.color AS category_color
        FROM todos t
        LEFT JOIN categories c
        ON c.id = t.category_id
        "
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn get_todo_db(pool: &PgPool, id: i32) -> Result<TodoWithoutCategoryDb, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoWithoutCategoryDb,
        "
        SELECT completed, label, id, description
        FROM todos
        WHERE id = $1
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn create_todo_db(pool: &PgPool, label: &String) -> Result<TodoDto, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoWithoutCategoryDb,
        "
        INSERT INTO todos(completed, label)
        VALUES($1, $2)
        RETURNING id, label, description, completed
        ",
        false,
        label
    )
    .fetch_one(pool)
    .await?;

    Ok(todo.to_dto())
}

pub async fn update_todo_db(
    pool: &PgPool,
    id: i32,
    completed: bool,
) -> Result<TodoWithoutCategoryDb, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoWithoutCategoryDb,
        "
        UPDATE todos
        SET completed = $1
        WHERE id = $2
        RETURNING id, description, completed, label
        ",
        completed,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn assign_todo_to_category_db(
    pool: &PgPool,
    todo_id: i32,
    category_id: i32,
) -> Result<TodoWithoutCategoryDb, Error> {
    let todo = sqlx::query_as!(
        TodoWithoutCategoryDb,
        "
        UPDATE todos
        SET category_id = $1 WHERE id = $2
        RETURNING id, completed, description, label
        ",
        category_id,
        todo_id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn delete_todo_db(pool: &PgPool, id: i32) -> Result<TodoWithoutCategoryDb, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoWithoutCategoryDb,
        "
        DELETE FROM todos WHERE id = $1
        RETURNING id, completed, description, label
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}
