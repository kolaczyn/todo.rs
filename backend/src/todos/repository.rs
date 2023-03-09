use sqlx::PgPool;

use super::dto::TodoDto;

pub async fn get_todos_db(pool: &PgPool) -> Result<Vec<TodoDto>, sqlx::Error> {
    let todos_db = sqlx::query!("SELECT completed, label, id, description FROM todos")
        .fetch_all(pool)
        .await?;

    let todos = todos_db
        .iter()
        .map(|x| TodoDto {
            id: x.id,
            label: x.label.to_owned(),
            completed: x.completed,
            description: x.description.to_owned(),
        })
        .collect();

    Ok(todos)
}

pub async fn get_todo_db(pool: &PgPool, id: i32) -> Result<TodoDto, sqlx::Error> {
    let todo = sqlx::query!(
        "
        SELECT completed, label, id, description
        FROM todos
        WHERE id = $1
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(TodoDto {
        id: todo.id,
        label: todo.label,
        completed: todo.completed,
        description: todo.description,
    })
}

pub async fn create_todo_db(pool: &PgPool, label: String) -> Result<TodoDto, sqlx::Error> {
    let result = sqlx::query!(
        r#"INSERT INTO todos(completed, label) VALUES($1, $2) RETURNING id"#,
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
        r#"UPDATE todos SET completed = $1 WHERE id = $2"#,
        new_todo.completed,
        new_todo.id
    )
    .execute(pool)
    .await?;

    let todo = get_todo_db(pool, new_todo.id).await?;

    Ok(todo)
}

pub async fn delete_todo_db(pool: &PgPool, id: i32) -> Result<TodoDto, sqlx::Error> {
    let todo = get_todo_db(pool, id).await?;
    sqlx::query!(r#"DELETE FROM todos WHERE id = $1"#, id)
        .execute(pool)
        .await?;

    Ok(todo)
}
