use actix_web::{web, HttpResponse, Result};

use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};

use crate::db::todo::repository;
use crate::db::AppState;

use super::dto::response::TodoQuery;

#[derive(Debug, Display, Error, From)]
pub enum TodoError {
    MysqlError(rbatis::Error),
    Unknown,
}

impl actix_web::ResponseError for TodoError {
    fn status_code(&self) -> StatusCode {
        match self {
            TodoError::MysqlError(_) | TodoError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn get_todo(id: i32, data: web::Data<AppState>) -> Result<TodoQuery, TodoError> {
    let todoRepo = repository::TodoRepository::new(data.mysql_pool.clone());
    let todo = todoRepo.select_todo(id).await;
    match todo {
        Ok(todo) => match todo {
            Some(todo) => Ok(TodoQuery {
                content: todo.content.unwrap(),
                done: todo.done.unwrap(),
            }),
            None => Err(TodoError::Unknown),
        },
        Err(e) => Err(TodoError::MysqlError(e)),
    }
}
