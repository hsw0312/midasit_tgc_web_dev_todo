use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub id: i32,
    pub content: String,
    pub done: i8,
}

#[derive(Deserialize)]
pub struct UpdateTodoContentRequest {
    pub id: i32,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoDoneRequest {
    pub id: i32,
    pub done: i8,
}
