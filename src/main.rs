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
            .configure(todo::config)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
