use actix_web::{web, App, HttpServer};
use db::{conn::create_mysql_pool, AppState};
use midasit_tgc_web_dev_todo::*;
mod db;
mod todo;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let mysql_pool = create_mysql_pool().await;
    let app_state = web::Data::new(AppState { mysql_pool });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(health_config)
            .service(todo::controller::get_todo)
            .service(todo::controller::get_done_todos)
            .service(todo::controller::get_todos)
            .service(todo::controller::post_todo)
            .service(todo::controller::put_todo)
            .service(todo::controller::delete_todo_by_id)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
