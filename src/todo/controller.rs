use actix_web::{error, get, web, HttpResponse, Responder};

use crate::{db::AppState, todo::service};

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
