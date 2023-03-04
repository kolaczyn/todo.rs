mod commands;
mod todo;
mod utils;

use clap::{arg, Command};
use commands::{
    read_command::{read_all_command, read_one_command},
    write_command::write_command,
};
use serde_json::Error;

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
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("read-all", _)) => read_all_command(),
        Some(("read", sub_matches)) => {
            // I think force unwraping makes sense here, because this is the outermost edge of the program
            // But there might be a better way of doing this
            let id = sub_matches.get_one::<String>("TODO_ID").unwrap();
            read_one_command(String::from(id));
        }
        Some(("write", sub_matches)) => {
            // I think force unwraping makes sense here, because this is the outermost edge of the program
            // But there might be a better way of doing this
            let label = sub_matches.get_one::<String>("TODO_LABEL").unwrap();
            write_command(String::from(label)).unwrap();
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn main() -> () {
    run_cli().unwrap();
    // establish_connection();
    println!();
}
