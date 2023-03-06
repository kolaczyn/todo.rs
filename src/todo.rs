use serde::{Deserialize, Serialize};
use std::cmp::Eq;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Todo {
    pub completed: bool,
    pub label: String,
    pub id: i32,
}

pub struct NewTodo {
    pub label: String,
}

impl Todo {
    pub fn new(label: String) -> Todo {
        Todo {
            completed: false,
            id: Todo::random_id(),
            label,
        }
    }
    pub fn random_id() -> i32 {
        rand::random()
    }
    pub fn to_string(&self) -> String {
        let check = if self.completed { "[x}" } else { "[ ]" };
        format!("{} {} #{}", check, self.label, self.id)
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
