mod file_utils;
mod generic_error;
mod random_id;
mod todo;
mod todo_notes;

use file_utils::{read_todo_from_file, write_todo_to_file};
use serde_json::Error;
use todo::Todo;

fn main() -> Result<(), Error> {
    let args = std::env::args().nth(1);

    match args {
        Some(x) => {
            let todo = Todo::new(x);
            write_todo_to_file(todo);
        }
        None => {
            let todo = read_todo_from_file();
            match todo {
                Ok(x) => print!("{:?}", x),
                Err(_) => (),
            }
        }
    }

    return Ok(());
}
