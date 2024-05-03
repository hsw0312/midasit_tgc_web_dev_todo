use actix_web::{get, post, web, Responder};

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
    web::block(move || {
        service::post_todo(TodoDto::new_from_request(data.into_inner()), app_state)
    })
    .await?
    .await?;

    Ok("".to_string())
}
