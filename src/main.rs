use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{ConnectOptions, PgPool};
mod controller;
mod db;
mod models;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);

    let pool = PgPool::connect(&database_url).await?;

    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(logger)
            .service(controller::review_controller::get_review)
            .service(controller::review_controller::add_review)
    })

    .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}