use crate::{file_utils::write_todo_to_file, todo::Todo};

pub fn write_command(label: String) {
    let todo = Todo::new(label.to_string());
    write_todo_to_file(todo).unwrap();
}
