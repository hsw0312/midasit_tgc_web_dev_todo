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
