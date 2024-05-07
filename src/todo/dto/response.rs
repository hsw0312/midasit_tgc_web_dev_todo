use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoQuery {
    pub content: String,
    pub done: i8,
}
