use crate::{
    file_utils::{read_todos_from_file, write_todos_to_file},
    generic_error::GenericError,
    todo::Todo,
};

pub fn write_command(label: String) -> Result<(), GenericError> {
    let todo = Todo::new(label.to_string());
    let mut todos = read_todos_from_file().unwrap_or(vec![]);
    todos.push(todo);
    write_todos_to_file(todos).unwrap();
    Ok(())
}
