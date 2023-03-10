use anyhow::Error;
use sqlx::PgPool;

use crate::todos::db_dto::TodoDb;

use super::dto::TodoDto;

pub async fn get_todos_db(pool: &PgPool) -> Result<Vec<TodoDto>, sqlx::Error> {
    let todos_db = sqlx::query_as!(
        TodoDb,
        r#"SELECT completed, label, id, description
    FROM todos"#
    )
    .fetch_all(pool)
    .await?;

    let todos = todos_db.iter().map(|x| x.to_dto()).collect();

    Ok(todos)
}

pub async fn get_todo_db(pool: &PgPool, id: i32) -> Result<TodoDto, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoDb,
        "
        SELECT completed, label, id, description
        FROM todos
        WHERE id = $1
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo.to_dto())
}

pub async fn create_todo_db(pool: &PgPool, label: String) -> Result<TodoDto, sqlx::Error> {
    let result = sqlx::query!(
        r#"INSERT INTO todos(completed, label)
        VALUES($1, $2)
        RETURNING id"#,
        false,
        label
    )
    .fetch_one(pool)
    .await?;

    // TODO this could be rewritten more efficiently - RETURNING could return the whole todo
    // But I should create a mapper or something similar
    let todo = get_todo_db(pool, result.id).await?;
    Ok(todo)
}

pub async fn update_todo_db(pool: &PgPool, new_todo: TodoDto) -> Result<TodoDto, sqlx::Error> {
    sqlx::query!(
        r#"UPDATE todos
        SET completed = $1
        WHERE id = $2"#,
        new_todo.completed,
        new_todo.id
    )
    .execute(pool)
    .await?;

    let todo = get_todo_db(pool, new_todo.id).await?;

    Ok(todo)
}

pub async fn assign_todo_to_category_db(
    pool: &PgPool,
    todo_id: i32,
    category_id: i32,
) -> Result<TodoDto, Error> {
    let todo_db = sqlx::query_as!(
        TodoDb,
        r#"UPDATE todos
        SET category_id = $1 WHERE id = $2
        RETURNING id, completed, description, label"#,
        category_id,
        todo_id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo_db.to_dto())
}

pub async fn delete_todo_db(pool: &PgPool, id: i32) -> Result<TodoDto, sqlx::Error> {
    let todo = get_todo_db(pool, id).await?;
    sqlx::query!(r#"DELETE FROM todos WHERE id = $1"#, id)
        .execute(pool)
        .await?;

    Ok(todo)
}
