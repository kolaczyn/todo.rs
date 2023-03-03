mod file_utils;
mod generic_error;
mod random_id;
mod todo;
mod todo_notes;

use clap::{arg, Command};
use file_utils::{read_todo_from_file, write_todo_to_file};
use serde_json::Error;
use todo::Todo;

fn cli() -> Command {
    Command::new("todo.rs")
        .about("A primitive tool for creating notes :p")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("read").about("Read stored todos"))
        .subcommand(
            Command::new("write")
                .about("Write todo")
                .arg(arg!(<TODO_LABEL> "Todo label")),
        )
}

fn main() -> Result<(), Error> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("read", _)) => {
            let todo = read_todo_from_file();
            match todo {
                Ok(x) => print!("{:?}", x),
                Err(_) => (),
            }
        }
        Some(("write", sub_matches)) => {
            // I think force unwraping makes sense here, because this is the outermost edge of the program
            // But there might be a better way of doing this
            let label = sub_matches.get_one::<String>("TODO_LABEL").unwrap();
            let todo = Todo::new(label.to_string());
            write_todo_to_file(todo).unwrap();
        }
        _ => unreachable!(),
    }

    return Ok(());
}
