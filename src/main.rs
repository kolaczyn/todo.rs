mod todo;

use dotenv::dotenv;
use sqlx::SqlitePool;
use std::env;
use tide::Request;
use todo::{CreateTodoDto, UpdateTodoDto};

use crate::todo::Todo;

async fn get_todos_db(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos: Vec<Todo> = sqlx::query_as("SELECT completed, label, id FROM todos")
        .fetch_all(pool)
        .await?;

    Ok(todos)
}

async fn get_todos_endpoint(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().connection_pool.clone();
    let todos = get_todos_db(&pool).await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn get_todo_db(pool: &SqlitePool, id: i32) -> Result<Todo, sqlx::Error> {
    let todos: Todo = sqlx::query_as(
        r#"
        SELECT completed, label, id
        FROM todos
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(todos)
}

async fn get_todo_endpoint(req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let pool = req.state().connection_pool.clone();

    let todos = get_todo_db(&pool, id).await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn create_todo_db(pool: &SqlitePool, label: String) -> Result<(), sqlx::Error> {
    let _rows_affected = sqlx::query("INSERT INTO todos(completed, label) VALUES($1, $2)")
        .bind(false)
        .bind(label)
        .execute(pool)
        .await;

    Ok(())
}

async fn create_todo_endpoint(mut req: Request<State>) -> tide::Result<String> {
    let label = req.body_json::<CreateTodoDto>().await?.label;
    let pool = req.state().connection_pool.clone();

    let result = create_todo_db(&pool, label).await;

    match result {
        Ok(_) => Ok(String::from("Done")),
        Err(_) => Ok(String::from("Something went wrong")),
    }
}

async fn update_todo_db(pool: &SqlitePool, new_todo: Todo) -> Result<(), sqlx::Error> {
    let _rows_affected = sqlx::query("UPDATE todos SET completed = $1 WHERE id = $2")
        .bind(new_todo.completed)
        .bind(new_todo.id)
        .execute(pool)
        .await;

    Ok(())
}

async fn update_todo_endpoint(mut req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let completed = req.body_json::<UpdateTodoDto>().await?.completed;
    let pool = req.state().connection_pool.clone();

    let todo = get_todo_db(&pool, id).await?;
    let new_todo = Todo { completed, ..todo };

    let result = update_todo_db(&pool, new_todo).await;
    match result {
        Ok(_) => Ok(String::from("Done")),
        Err(_) => Ok(String::from("Something went wrong")),
    }
}

#[derive(Clone)]
struct State {
    connection_pool: SqlitePool,
}

impl State {
    async fn new(database_url: String) -> Result<Self, sqlx::Error> {
        Ok(Self {
            connection_pool: SqlitePool::connect(&database_url).await?,
        })
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv()?;
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let state = State::new(database_url).await?;
    let mut app = tide::with_state(state);

    tide::log::start();

    app.at("/todos").get(get_todos_endpoint);
    app.at("/todos/:id").get(get_todo_endpoint);
    app.at("/todos").post(create_todo_endpoint);
    app.at("/todos/:id").patch(update_todo_endpoint);

    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}
