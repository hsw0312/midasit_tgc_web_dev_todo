use actix_web::{get, post, delete, put, web, Responder};
use serde_json::json; // 추가: serde_json::json 매크로를 임포트

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
    println!("Received id: {:?}", id);

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

#[delete("/todo/{id}")]
pub async fn delete_todo(
    id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let todo_id = id.into_inner();  // Extract the id from the URL path
    
    println!("Received id: {:?}", todo_id);
    
    let result = web::block(move || service::delete_todo(todo_id, app_state)).await?;

    match result.await {
        Ok(()) => Ok(web::Json(json!({"message": "Successfully deleted"}))),
        Err(_) => Ok(web::Json(json!({"message": "Failed to delete"}))),
    }
}

#[put("/todo")]
pub async fn put_todo(
    data: web::Json<crate::todo::dto::request::UpdateTodoRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let todo_id = data.id;
    let content = data.content.clone();
    let done = data.done;

    match service::update_todo_by_id(todo_id, content, done, &app_state).await {
        Ok(_) => web::Json(json!({"message": "Todo updated successfully"})),
        Err(_) => web::Json(json!({"message": "Failed to update todo"})),
    }
}
