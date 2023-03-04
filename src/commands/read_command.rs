use crate::{todo::Todo, utils::file_utils::read_todos_from_file};

pub fn read_all_command() -> () {
    let todos = read_todos_from_file();
    match todos {
        Ok(todos) => todos
            .iter()
            .map(|todo| todo.to_string())
            .for_each(|s| println!("{}", s)),
        Err(_) => print!("You have no todos"),
    }
}

fn find_todo<'a>(todo_id: String, todos: &'a Vec<Todo>) -> Option<&'a Todo> {
    todos.iter().find(|x| x.legacy_id == todo_id)
}

const TODO_NOT_FOUND_MESSAGE: &str = "Couldn't find todo with the specified id";

pub fn read_one_command(todo_id: String) -> () {
    let todos = read_todos_from_file();
    // nested todos look fucking ugly, but I dunno how to do it idiomatically :p
    match todos {
        Ok(x) => {
            let todo = find_todo(todo_id, &x);
            match todo {
                Some(todo) => print!("{}", todo.to_string()),
                None => println!("{}", TODO_NOT_FOUND_MESSAGE),
            }
        }
        Err(_) => println!("{}", TODO_NOT_FOUND_MESSAGE),
    }
}
