use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub content: String,
    pub done: i8,
}

#[derive(Deserialize)]
pub struct UpdateDone {
    pub done: i8,
}