extern crate rbatis;

pub mod db;
pub mod todo;
use actix_web::{get, head, HttpResponse, Responder, Result};
use serde::Serialize;

pub fn health_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(head_health).service(get_health).service(hello);
}

#[head("/health.html")]
async fn head_health() -> impl Responder {
    HttpResponse::Ok().body("true")
}

#[get("/health.html")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("health : true")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize)]
pub struct Response {
    meesage: String,
}

pub async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        meesage: "resource not found".to_string(),
    };

    Ok(HttpResponse::NotFound().json(response))
}
