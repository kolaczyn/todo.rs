use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub completed: bool,
    pub label: String,
    pub id: i64,
}

impl Todo {
    pub fn new(label: String) -> Todo {
        Todo {
            completed: false,
            id: rand::random::<i64>(),
            label,
        }
    }
}
