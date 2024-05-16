use actix_web::{get, post, put, delete, web, Responder};

use crate::{
    db::AppState,
    todo::{dto::todo::TodoDto, service},
};

#[get("/todo/{id}")]
pub async fn get_todo(
    id: web::Path<i32>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let todo = web::block(move || service::get_todo(id.into_inner(), data))
        .await?
        .await?;
    Ok(web::Json(todo))
}

#[get("/todo/done/{done}")]
pub async fn get_done_todos(
    done: web::Path<i8>,
    data: web::Data<AppState>
) -> actix_web::Result<impl Responder> {
    let todos = web::block(move || service::get_done_todos(done.into_inner(), data)).await?.await?;
    Ok(web::Json(todos))
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

#[put("/todo/{id}")]
pub async fn put_todo(
    id: web::Path<i32>,
    data: web::Json<crate::todo::dto::request::UpdateTodoRequest>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    web::block(move || service::put_todo(TodoDto::combination_from_request_for_both(id.into_inner(), data.into_inner()), app_state))
        .await?
        .await?;

    Ok("".to_string())
}

#[put("/todo/content/{id}")]
pub async fn put_todo_content(
    id: web::Path<i32>,
    data: web::Json<crate::todo::dto::request::CreateTodoRequest>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    web::block(move || service::put_todo_content(TodoDto::combination_from_request_for_content(id.into_inner(), data.into_inner()), app_state))
        .await?
        .await?;

    Ok("".to_string())
}

#[put("/todo/done/{id}")]
pub async fn put_todo_done(
    id: web::Path<i32>,
    data: web::Json<crate::todo::dto::request::UpdateDone>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    web::block(move || service::put_todo_done(TodoDto::combination_from_request_for_done(id.into_inner(), data.into_inner()), app_state))
        .await?
        .await?;

    Ok("".to_string())
}

#[delete("/todo/{id}")]
pub async fn delete_todo_by_id(
    id: web::Path<i32>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    web::block(move || service::delete_todo_by_id(id.into_inner(), data))
        .await?
        .await?;

    Ok("".to_string())
}