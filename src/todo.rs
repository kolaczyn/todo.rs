use serde::{Deserialize, Serialize};
use std::cmp::Eq;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Todo {
    pub completed: bool,
    pub label: String,
    pub legacy_id: String,
}

impl Todo {
    pub fn new(label: String) -> Todo {
        Todo {
            completed: false,
            legacy_id: Todo::random_id(),
            label,
        }
    }
    pub fn random_id() -> String {
        let word_one = random_word::gen();
        let word_two = random_word::gen();
        format!("{} {}", word_one, word_two)
    }
    pub fn to_string(&self) -> String {
        let check = if self.completed { "[x}" } else { "[ ]" };
        format!("{} {} #{}", check, self.label, self.legacy_id)
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
