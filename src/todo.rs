use serde::{Deserialize, Serialize};
use std::cmp::Eq;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Todo {
    pub completed: bool,
    pub label: String,
    pub id: i32,
}
