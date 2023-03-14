use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
pub struct TodoDto {
    pub id: i32,
    pub label: String,
    pub completed: bool,
}

#[get("/")]
fn get_todos() -> Json<TodoDto> {
    let todo = TodoDto {
        completed: false,
        id: 1,
        label: String::from("Hello world!"),
    };
    Json(todo)
}

#[get("/1")]
fn get_todo() -> Json<Vec<TodoDto>> {
    let todo = TodoDto {
        completed: false,
        id: 1,
        label: String::from("Hello world!"),
    };
    let todos = vec![todo];
    Json(todos)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1/todos", routes![get_todo, get_todos])
}
