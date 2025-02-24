use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;

mod controller;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);


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