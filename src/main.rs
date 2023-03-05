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
    associations::HasTable, query_dsl::methods::FilterDsl, QueryDsl, RunQueryDsl, SqliteConnection,
};
use serde_json::Error;

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

fn main() -> Result<(), Error> {
    // run_cli()?;
    // println!();
    // Ok(())

    diesel_experiments()
}
