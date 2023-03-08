use serde::{Deserialize, Serialize};
use std::cmp::Eq;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoDto {
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoDto {
    pub completed: bool,
}
