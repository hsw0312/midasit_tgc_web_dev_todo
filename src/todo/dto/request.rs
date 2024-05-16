use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct PutTodoRequest {
    pub id: i32,
    pub content: String,
    pub done: i8,
}

#[derive(Deserialize)]
pub struct PutTodoContentRequest {
    pub id: i32,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PutTodoDoneRequest {
    pub id: i32,
    pub done: i8,
}