mod commands;
mod schema;
mod todo;
mod utils;

use clap::{arg, Command};
use commands::{
    read_command::{read_all_command, read_one_command},
    write_command::write_command,
};
use diesel::{
    associations::HasTable, query_dsl::methods::FilterDsl, serialize, QueryDsl, RunQueryDsl,
    SqliteConnection,
};
use serde::Serialize;
use serde_json::Error;
use tide::{Body, Request, Response};

use crate::{
    todo::{NewTodo, Todo},
    utils::establish_connection::establish_connection,
};

fn cli() -> Command {
    Command::new("todo.rs")
        .about("A primitive tool for creating notes :p")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("read-all").about("Read all todos"))
        .subcommand(
            Command::new("read")
                .about("Read selected todo")
                .arg(arg!(<TODO_ID> "Todo id")),
        )
        .subcommand(
            Command::new("write")
                .about("Write todo")
                .arg(arg!(<TODO_LABEL> "Todo label")),
        )
}

fn run_cli() -> Result<(), Error> {
    let mut conn = establish_connection();
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("read-all", _)) => read_all_command(conn),
        Some(("read", sub_matches)) => {
            // I think force unwraping makes sense here, because this is the outermost edge of the program
            // But there might be a better way of doing this
            let id = sub_matches.get_one::<String>("TODO_ID").unwrap();
            read_one_command(conn, String::from(id));
        }
        Some(("write", sub_matches)) => {
            // I think force unwraping makes sense here, because this is the outermost edge of the program
            // But there might be a better way of doing this
            let label = sub_matches.get_one::<String>("TODO_LABEL").unwrap();
            write_command(conn, String::from(label)).unwrap();
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn add_todo() {
    let mut conn = establish_connection();
    use self::schema::todos::dsl::*;

    let new_todo = NewTodo {
        label: String::from("Hello world!"),
    };

    diesel::insert_into(todos)
        .values(&new_todo)
        .execute(&mut conn)
        .expect("Couldn't save the post");
}

fn diesel_experiments() -> Result<(), Error> {
    use self::schema::todos::dsl::*;
    let mut conn = establish_connection();
    add_todo();
    let found_todos = todos.load::<Todo>(&mut conn).unwrap_or(vec![]);
    found_todos
        .iter()
        .for_each(|todo| println!("{}", todo.to_string()));
    Ok(())
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
    Ok(serde_json::to_string_pretty(&todos())?)
}

async fn get_todo(req: Request<()>) -> tide::Result<String> {
    let id: i32 = req.param("id")?.parse().unwrap();
    let all_todos = todos();
    let todos = all_todos.iter().find(|x| x.id == id);

    match todos {
        Some(x) => Ok(serde_json::to_string(&x)?),
        // This is not okay, will have to fix this :p
        None => Ok(String::from("Not found")),
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/hello").get(get_hello_word);
    app.at("/todos").get(get_todos);
    app.at("/todos/:id").get(get_todo);

    println!("Listening on http://localhost:8080");
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
