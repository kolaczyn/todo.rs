use serde::{Deserialize, Serialize};

use crate::random_id::random_id;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub completed: bool,
    pub label: String,
    pub id: String
}

impl Todo {
    pub fn new(label: String) -> Todo {
        Todo {
            completed: false,
            id: random_id(),
            label,
        }
    }
}
