mod todo;

use sqlx::{Connection, SqliteConnection};
use tide::Request;

use crate::todo::Todo;

async fn get_connection() -> Result<SqliteConnection, sqlx::Error> {
    SqliteConnection::connect("sqlite:db.sql").await
}

fn todos() -> Vec<Todo> {
    let todo_one = Todo {
        completed: false,
        id: 1,
        label: String::from("Learn Rust"),
    };

    let todo_two = Todo {
        completed: false,
        id: 2,
        label: String::from("Learn Rust"),
    };

    vec![todo_one, todo_two]
}

async fn get_hello_word(_req: Request<()>) -> tide::Result<String> {
    Ok(String::from("Hello world"))
}

async fn get_todos(_req: Request<()>) -> tide::Result<String> {
    let mut conn = get_connection().await?;
    let todos = sqlx::query_as::<_, Todo>("SELECT completed, label, id FROM todos")
        .fetch_all(&mut conn)
        .await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn get_todo(req: Request<()>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let mut conn = get_connection().await?;
    // THIS COULD CAUSE AN SQL INJECTION! I should probably fix this :p
    let query = format!("SELECT completed, label, id FROM todos WHERE id = {}", id).to_owned();
    let todos = sqlx::query_as::<_, Todo>(&query)
        .fetch_all(&mut conn)
        .await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn create_todo(req: Request<()>) -> tide::Result<String> {
    println!("Bifor");
    let label: String = req.param("label")?.parse()?;
    let mut conn = get_connection().await?;
    // THIS WILL SURELY CAUSE AN SQL INJECTION! I should probably fix this :p
    let query = format!(
        "INSERT INTO todos(completed, label) VALUES({}, '{}')",
        false, label
    )
    .to_owned();
    let rows_affected = sqlx::query(&query).execute(&mut conn).await;

    match rows_affected {
        Ok(_) => Ok(String::from("Done")),
        Err(_) => Ok(String::from("Something went wrong")),
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/hello").get(get_hello_word);
    app.at("/todos").get(get_todos);
    app.at("/todos/:id").get(get_todo);
    // this shouldn't be a GET, but whatever :p
    app.at("/create-todo/:label").get(create_todo);

    println!("Listening on http://localhost:8080");
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
