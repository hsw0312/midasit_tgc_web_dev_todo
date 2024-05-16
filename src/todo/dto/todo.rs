use serde::{Deserialize, Serialize};

use super::request::CreateTodoRequest;
use super::request::UpdateTodoRequest;
use super::request::UpdateDone;

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

    pub fn combination_from_request_for_both(key: i32, rq: UpdateTodoRequest) -> Self {
        Self {
            id: key,
            content: rq.content.clone(),
            done: rq.done,
        }
    }

    pub fn combination_from_request_for_content(key: i32, rq: CreateTodoRequest) -> Self {
        Self {
            id: key,
            content: rq.content.clone(),
            done: 0,
        }
    }

    pub fn combination_from_request_for_done(key: i32, rq: UpdateDone) -> Self {
        Self {
            id: key,
            content: "".to_string(),
            done: rq.done,
        }
    }
}
