mod commands;
mod todo;
mod utils;

use clap::{arg, Command};
use commands::{read_command::read_command, write_command::write_command};
use serde_json::Error;

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
        Some(("read", _)) => read_command(),
        Some(("write", sub_matches)) => {
            // I think force unwraping makes sense here, because this is the outermost edge of the program
            // But there might be a better way of doing this
            let label = sub_matches.get_one::<String>("TODO_LABEL").unwrap();
            write_command(String::from(label)).unwrap();
        }
        _ => unreachable!(),
    }

    return Ok(());
}
