use crate::todo::Todo;

use super::generic_error::GenericError;

const NOTES_FILE_NAME: &str = "todos.notes";

pub fn read_todos_from_file() -> Result<Vec<Todo>, GenericError> {
    let as_string = std::fs::read_to_string(NOTES_FILE_NAME)?;
    let todos = serde_json::from_str::<Vec<Todo>>(&as_string)?;
    Ok(todos)
}

pub fn write_todos_to_file(todos: Vec<Todo>) -> Result<(), GenericError> {
    let as_string = serde_json::to_string_pretty(&todos)?;
    std::fs::write(NOTES_FILE_NAME, as_string)?;
    Ok(())
}
