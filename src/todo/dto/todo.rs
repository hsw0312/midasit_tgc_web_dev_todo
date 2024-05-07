use serde::{Deserialize, Serialize};

use super::request::{CreateTodoRequest, UpdateTodoRequest};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDto {
    pub id: i32,
    pub content: String,
    pub done: i8,
}

impl TodoDto {
    // pub fn new(id: i32, content: String, done: i8) -> Self {
    //     Self { id, content, done }
    // }

    pub fn new_from_request(rq: CreateTodoRequest) -> Self {
        Self {
            id: 0,
            content: rq.content.clone(),
            done: 0,
        }
    }

    pub fn update_from_request(rq: UpdateTodoRequest) -> Self {
        Self {
            id: rq.id,
            content: rq.content.clone(),
            done: rq.done,
        }
    }
}
