use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoQuery {
    pub id: i32,
    pub content: String,
    pub done: i8,
}
