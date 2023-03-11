use anyhow::Error;
use sqlx::PgPool;

use crate::todos::db_dto::TodoDb;

use super::dto::{TodoCategoryDto, TodoDto};

pub async fn get_todos_db(pool: &PgPool) -> Result<Vec<TodoDto>, sqlx::Error> {
    let todos_db = sqlx::query!(
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

    let todos = todos_db
        .iter()
        .map(|x| {
            let category = match (
                x.category_id,
                x.category_label.to_owned(),
                x.category_color.to_owned(),
            ) {
                (Some(c_id), Some(c_label), Some(c_color)) => Some(TodoCategoryDto {
                    id: c_id,
                    label: c_label,
                    color: c_color,
                }),
                _ => None,
            };
            return TodoDto {
                id: x.id,
                completed: x.completed,
                label: x.label.to_owned(),
                description: x.description.to_owned(),
                category,
            };
        })
        .collect();

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
    let todo = sqlx::query_as!(
        TodoDb,
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

pub async fn update_todo_db(pool: &PgPool, new_todo: TodoDto) -> Result<TodoDto, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoDb,
        "
        UPDATE todos
        SET completed = $1
        WHERE id = $2
        RETURNING id, description, completed, label
        ",
        new_todo.completed,
        new_todo.id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo.to_dto())
}

pub async fn assign_todo_to_category_db(
    pool: &PgPool,
    todo_id: i32,
    category_id: i32,
) -> Result<TodoDto, Error> {
    let todo = sqlx::query_as!(
        TodoDb,
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

    Ok(todo.to_dto())
}

pub async fn delete_todo_db(pool: &PgPool, id: i32) -> Result<TodoDto, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoDb,
        "
        DELETE FROM todos WHERE id = $1
        RETURNING id, completed, description, label
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo.to_dto())
}
