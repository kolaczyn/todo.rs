mod todo;

use dotenv::dotenv;
use sqlx::SqlitePool;
use std::env;
use tide::Request;
use todo::{CreateTodoDto, UpdateTodoDto};

use crate::todo::Todo;

async fn get_todos_db(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos_db = sqlx::query!("SELECT completed, label, id, description FROM todos")
        .fetch_all(pool)
        .await?;

    let todos = todos_db
        .iter()
        .map(|x| Todo {
            id: x.id,
            label: x.label.to_owned(),
            completed: x.completed,
            description: x.description.to_owned(),
        })
        .collect();

    Ok(todos)
}

async fn get_todos_endpoint(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().connection_pool.clone();
    let todos = get_todos_db(&pool).await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn get_todo_db(pool: &SqlitePool, id: i64) -> Result<Todo, sqlx::Error> {
    let todo = sqlx::query!(
        "
        SELECT completed, label, id, description
        FROM todos
        WHERE id = ?
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(Todo {
        id: todo.id,
        label: todo.label,
        completed: todo.completed,
        description: todo.description,
    })
}

async fn get_todo_endpoint(req: Request<State>) -> tide::Result<String> {
    let id: i64 = req.param("id")?.parse()?;
    let pool = req.state().connection_pool.clone();

    let todos = get_todo_db(&pool, id).await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn create_todo_db(pool: &SqlitePool, label: String) -> Result<Todo, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO todos(completed, label) VALUES(?1, ?2)",
        false,
        label
    )
    .execute(pool)
    .await?;

    let todo = get_todo_db(pool, result.last_insert_rowid()).await?;
    Ok(todo)
}

async fn create_todo_endpoint(mut req: Request<State>) -> tide::Result<String> {
    let label = req.body_json::<CreateTodoDto>().await?.label;
    let pool = req.state().connection_pool.clone();

    let todo = create_todo_db(&pool, label).await?;
    Ok(serde_json::to_string_pretty(&todo)?)
}

async fn update_todo_db(pool: &SqlitePool, new_todo: Todo) -> Result<Todo, sqlx::Error> {
    sqlx::query!(
        "UPDATE todos SET completed = ?1 WHERE id = ?2",
        new_todo.completed,
        new_todo.id
    )
    .execute(pool)
    .await?;

    let todo = get_todo_db(pool, new_todo.id).await?;

    Ok(todo)
}

async fn update_todo_endpoint(mut req: Request<State>) -> tide::Result<String> {
    let id: i64 = req.param("id")?.parse()?;
    let completed = req.body_json::<UpdateTodoDto>().await?.completed;
    let pool = req.state().connection_pool.clone();

    // yeah, the names here are pretty confusing :p
    let todo_db = get_todo_db(&pool, id).await?;
    let new_todo = Todo {
        completed,
        ..todo_db
    };
    let new_todo_db = update_todo_db(&pool, new_todo).await?;

    Ok(serde_json::to_string_pretty(&new_todo_db)?)
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
