mod todo;

use sqlx::SqlitePool;
use tide::Request;

use crate::todo::Todo;

async fn get_hello_word_endpoint(_req: Request<State>) -> tide::Result<String> {
    Ok(String::from("Hello world"))
}

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

async fn create_todo_endpoint(req: Request<State>) -> tide::Result<String> {
    let label: String = req.param("label")?.parse()?;
    let pool = req.state().connection_pool.clone();

    let result = create_todo_db(&pool, label).await;

    match result {
        Ok(_) => Ok(String::from("Done")),
        Err(_) => Ok(String::from("Something went wrong")),
    }
}

async fn set_todo_db(pool: &SqlitePool, new_todo: Todo) -> Result<(), sqlx::Error> {
    let _rows_affected = sqlx::query("UPDATE todos SET completed = $1 WHERE id = $2")
        .bind(new_todo.completed)
        .bind(new_todo.id)
        .execute(pool)
        .await;

    Ok(())
}

async fn set_todo_endpoint(req: Request<State>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse()?;
    let completed: bool = req.param("completed")?.parse()?;
    let pool = req.state().connection_pool.clone();

    let todo = get_todo_db(&pool, id).await?;
    let new_todo = Todo { completed, ..todo };

    let result = set_todo_db(&pool, new_todo).await;
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
    async fn new() -> Result<Self, sqlx::Error> {
        Ok(Self {
            // TODO should read connection string from env
            connection_pool: SqlitePool::connect("db.sql").await?,
        })
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = State::new().await?;
    let mut app = tide::with_state(state);

    tide::log::start();

    app.at("/hello").get(get_hello_word_endpoint);
    app.at("/todos").get(get_todos_endpoint);

    app.at("/todos/:id").get(get_todo_endpoint);
    // this shouldn't be a GET, but whatever :p
    app.at("/create-todo/:label").get(create_todo_endpoint);
    app.at("/set-todo/:id/:completed").get(set_todo_endpoint);

    println!("Listening on http://localhost:8080");
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
