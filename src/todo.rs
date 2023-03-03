use serde::{Deserialize, Serialize};
use std::cmp::Eq;

use crate::random_id::random_id;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Todo {
    pub completed: bool,
    pub label: String,
    pub id: String,
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

#[cfg(test)]
mod tests {
    use super::Todo;

    #[test]
    fn new_is_not_completed() {
        let todo = Todo::new(String::from("Hello world"));

        assert_eq!(todo.completed, false);
        assert_eq!(todo.label, "Hello world");
    }
}
