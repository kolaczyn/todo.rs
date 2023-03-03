use serde::{Deserialize, Serialize};

use crate::todo::Todo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoNotes {
    pub todo_list: Vec<Todo>,
}

impl TodoNotes {
    pub fn add_todo(&self, label: String) -> TodoNotes {
        // TODO will have to fix this
        let mut todo_list = self.todo_list.clone();
        todo_list.push(Todo::new(label));
        TodoNotes { todo_list }
    }

    pub fn print(&self) -> () {
        self.todo_list.iter().for_each(|x| println!("{}", x.label));
    }

    pub fn new() -> TodoNotes {
        TodoNotes { todo_list: vec![] }
    }

    pub fn example() -> TodoNotes {
        TodoNotes {
            todo_list: vec![
                Todo::new(String::from("Learn Rust")),
                Todo::new(String::from("Learn Rocket")),
            ],
        }
    }

}
