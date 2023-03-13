use sqlx::PgPool;

use crate::todos::repository::models::TodoDb;

pub async fn get_todos_db(pool: &PgPool) -> Result<Vec<TodoDb>, sqlx::Error> {
    let todos = sqlx::query_as!(
        TodoDb,
        "
        SELECT completed, label, id, description, category_id
        FROM todos
        "
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn get_todo_db(pool: &PgPool, id: i32) -> Result<TodoDb, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoDb,
        r#"
        SELECT completed, label, id, description, category_id
        FROM todos
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn create_todo_db(pool: &PgPool, label: &String) -> Result<TodoDb, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoDb,
        r#"
        INSERT INTO todos(completed, label)
        VALUES($1, $2)
        RETURNING id, label, description, completed, category_id
        "#,
        false,
        label
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn update_todo_db(
    pool: &PgPool,
    id: i32,
    completed: bool,
) -> Result<TodoDb, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoDb,
        "
        UPDATE todos
        SET completed = $1
        WHERE id = $2
        RETURNING id, description, completed, label, category_id
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
) -> Result<TodoDb, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoDb,
        "
        UPDATE todos
        SET category_id = $1 WHERE id = $2
        RETURNING id, completed, description, label, category_id
        ",
        category_id,
        todo_id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn delete_todo_db(pool: &PgPool, id: i32) -> Result<TodoDb, sqlx::Error> {
    let todo = sqlx::query_as!(
        TodoDb,
        "
        DELETE FROM todos WHERE id = $1
        RETURNING id, completed, description, label, category_id
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}
