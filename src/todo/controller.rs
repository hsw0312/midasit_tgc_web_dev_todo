use actix_web::{delete, get, post, put, web, Responder};

use crate::{
    db::AppState,
    todo::{dto::todo::TodoDto, service},
};

#[get("/todo/{id}")]
pub async fn get_todo(
    id: web::Path<i32>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    format!("Hello, {}!", id);

    let todo = web::block(move || service::get_todo(id.into_inner(), data))
        .await?
        .await?;
    Ok(web::Json(todo))
}

#[get("/todo")]
pub async fn get_todos(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let todos = web::block(move || service::get_todos(data)).await?.await?;
    Ok(web::Json(todos))
}

#[post("/todo")]
pub async fn post_todo(
    data: web::Json<crate::todo::dto::request::CreateTodoRequest>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    web::block(move || service::post_todo(TodoDto::new_from_request(data.into_inner()), app_state))
        .await?
        .await?;

    Ok("".to_string())
}

#[put("/todo")]
pub async fn put_todo(
    data: web::Json<crate::todo::dto::request::UpdateTodoRequest>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    web::block(move || {
        service::update_todo(TodoDto::update_from_request(data.into_inner()), app_state)
    })
    .await?
    .await?;

    Ok("".to_string())
}

#[delete("/todo/{id}")]
pub async fn delete_todo(
    id: web::Path<i32>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    web::block(move || service::delete_todo_by_id(id.into_inner(), data))
        .await?
        .await?;

    Ok("".to_string())
}
