use actix_identity::IdentityMiddleware;
use crate::repository::app_state::AppState;
use actix_web::web::Data;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::{PgPool};

mod controller;
mod models;
mod repository;
mod service;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);

    let pool = PgPool::connect(&database_url).await?;
    let app_state = Data::new(AppState::new(pool));

    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(app_state.clone())
            .wrap(IdentityMiddleware::default()) // Session-based middleware
            .wrap(logger)
            .service(controller::review_controller::add_review)
            .service(controller::review_controller::get_review)
            .service(controller::review_controller::get_all_reviews)
            .service(controller::review_controller::delete_review)
            .service(controller::user_controller::register_user)
            .service(controller::user_controller::me)
            .service(controller::user_controller::login)
            .service(controller::user_controller::get_user_by_id)
    })
    .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}