mod todo;

use std::sync::Arc;

use sqlx::{Connection, Pool, Sqlite, SqliteConnection, SqlitePool};
use tide::Request;

use crate::todo::Todo;

async fn get_hello_word(_req: Request<State>) -> tide::Result<String> {
    Ok(String::from("Hello world"))
}

async fn get_todos(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().connection_pool.clone();
    let todos: Vec<Todo> = sqlx::query_as("SELECT completed, label, id FROM todos")
        .fetch_all(&pool)
        .await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn get_todo(req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let pool = req.state().connection_pool.clone();

    let todos: Vec<Todo> = sqlx::query_as(
        r#"
        SELECT completed, label, id
        FROM todos
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_all(&pool)
    .await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn create_todo(req: Request<State>) -> tide::Result<String> {
    let label: String = req.param("label")?.parse()?;
    let pool = req.state().connection_pool.clone();

    let rows_affected = sqlx::query("INSERT INTO todos(completed, label) VALUES($1, $2)")
        .bind(false)
        .bind(label)
        .execute(&pool)
        .await;

    match rows_affected {
        Ok(_) => Ok(String::from("Done")),
        Err(_) => Ok(String::from("Something went wrong")),
    }
}

#[derive(Clone)]
struct State {
    connection_pool: Pool<Sqlite>,
}

impl State {
    async fn new() -> Result<Self, sqlx::Error> {
        let connection_pool = SqlitePool::connect("db.sql").await?;
        Ok(Self { connection_pool })
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = State::new().await?;
    let mut app = tide::with_state(state);

    app.with(tide::log::LogMiddleware::new());

    app.at("/hello").get(get_hello_word);
    app.at("/todos").get(get_todos);

    app.at("/todos/:id").get(get_todo);
    // this shouldn't be a GET, but whatever :p
    app.at("/create-todo/:label").get(create_todo);

    println!("Listening on http://localhost:8080");
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
