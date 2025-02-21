use actix_web::{middleware::Logger, App, HttpServer};
mod controller;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(controller::review_controller::review)
            .service(controller::review_controller::add_review)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}