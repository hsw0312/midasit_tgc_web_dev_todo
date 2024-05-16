use serde::{Deserialize, Serialize};

use super::request::CreateTodoRequest;
use super::request::UpdateTodoRequest;

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

    pub fn combination_from_request(key: i32, rq: UpdateTodoRequest) -> Self {
        Self {
            id: key,
            content: rq.content.clone(),
            done: rq.done,
        }
    }
}
