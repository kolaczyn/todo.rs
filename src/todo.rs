use serde::{Deserialize, Serialize};
use std::cmp::Eq;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Todo {
    pub completed: bool,
    pub label: String,
    pub id: i32,
}

impl Todo {
    pub fn to_string(&self) -> String {
        let check = if self.completed { "[x}" } else { "[ ]" };
        format!("{} {} #{}", check, self.label, self.id)
    }
}
