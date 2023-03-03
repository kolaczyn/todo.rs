use crate::{todo::Todo, generic_error::GenericError};

const NOTES_FILE_NAME: &str = "todos.notes";

pub fn read_todo_from_file() -> Result<Todo, GenericError> {
    let as_string = std::fs::read_to_string(NOTES_FILE_NAME)?;
    let todo = serde_json::from_str::<Todo>(&as_string)?;
    Ok(todo)
}

pub fn write_todo_to_file(todo: Todo) -> Result<(), GenericError> {
    let as_string = serde_json::to_string_pretty(&todo)?;
    std::fs::write(NOTES_FILE_NAME, as_string);
    Ok(())
}
