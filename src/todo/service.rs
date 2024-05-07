use actix_web::{web, Result};

use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};

use crate::db::todo::repository;
use crate::db::AppState;

use super::dto::response::TodoQuery;

#[derive(Debug, Display, Error, From)]
pub enum TodoError {
    MysqlError(rbatis::Error),
    NotFound,
    Unknown,
}

impl actix_web::ResponseError for TodoError {
    fn status_code(&self) -> StatusCode {
        match self {
            TodoError::MysqlError(_) | TodoError::NotFound => StatusCode::NOT_FOUND,
            TodoError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn get_todo(id: i32, data: web::Data<AppState>) -> Result<TodoQuery, TodoError> {
    let todo_repo = repository::TodoRepository::new(data.mysql_pool.clone());
    let todo = todo_repo.select_todo(id).await;
    match todo {
        Ok(todo) => match todo {
            Some(todo) => Ok(TodoQuery {
                id: todo.id,
                content: todo.content.unwrap(),
                done: todo.done.unwrap(),
            }),
            None => Err(TodoError::Unknown),
        },
        Err(e) => Err(TodoError::MysqlError(e)),
    }
}

pub async fn get_todos(data: web::Data<AppState>) -> Result<Vec<TodoQuery>, TodoError> {
    let todo_repo = repository::TodoRepository::new(data.mysql_pool.clone());
    let todos = todo_repo.select_all().await;
    match todos {
        Ok(todos) => {
            let mut todo_queries = Vec::new();
            for todo in todos {
                todo_queries.push(TodoQuery {
                    id: todo.id,
                    content: todo.content.unwrap(),
                    done: todo.done.unwrap(),
                });
            }
            Ok(todo_queries)
        }
        Err(e) => Err(TodoError::MysqlError(e)),
    }
}

pub async fn post_todo(
    todo: crate::todo::dto::todo::TodoDto,
    app_state: web::Data<AppState>,
) -> Result<(), TodoError> {
    let todo_repo = repository::TodoRepository::new(app_state.mysql_pool.clone());

    let todo = todo_repo
        .insert_into_todo(crate::db::todo::schema::Todo {
            id: 0,
            content: Some(todo.content),
            done: Some(todo.done),
        })
        .await;

    match todo {
        Ok(_) => Ok(()),
        Err(e) => Err(TodoError::MysqlError(e)),
    }
}

pub async fn delete_todo_by_id(id: i32, data: web::Data<AppState>) -> Result<(), TodoError> {
    let todo_repo = repository::TodoRepository::new(data.mysql_pool.clone());

    match todo_repo.select_todo(id).await {
        Ok(Some(_)) => (),
        Ok(None) => return Err(TodoError::NotFound),
        Err(e) => return Err(TodoError::MysqlError(e)),
    }

    let todo = todo_repo.delete_todo_by_id(id).await;
    match todo {
        Ok(_) => Ok(()),
        Err(e) => Err(TodoError::MysqlError(e)),
    }
}

pub async fn update_todo(
    todo: crate::todo::dto::todo::TodoDto,
    app_state: web::Data<AppState>,
) -> Result<(), TodoError> {
    let todo_repo = repository::TodoRepository::new(app_state.mysql_pool.clone());

    match todo_repo.select_todo(todo.id).await {
        Ok(Some(_)) => (),
        Ok(None) => return Err(TodoError::NotFound),
        Err(e) => return Err(TodoError::MysqlError(e)),
    }

    let todo = todo_repo
        .put_todo(crate::db::todo::schema::Todo {
            id: todo.id,
            content: Some(todo.content),
            done: Some(todo.done),
        })
        .await;

    match todo {
        Ok(_) => Ok(()),
        Err(e) => Err(TodoError::MysqlError(e)),
    }
}
