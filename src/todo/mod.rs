use actix_web::web;

pub mod controller;
pub mod dto;
pub mod service;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(controller::get_todo)
        .service(controller::get_todos)
        .service(controller::post_todo)
        .service(controller::put_todo)
        .service(controller::delete_todo);
}
