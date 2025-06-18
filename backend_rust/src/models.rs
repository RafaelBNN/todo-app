use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub done: bool,
}
