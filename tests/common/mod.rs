use testcontainers::{clients, images::postgres::Postgres};
use sqlx::{PgPool, Pool, Postgres};
use std::env;
//TODO please check why I cant use images for the containers?
pub struct TestDb {
    pub pool: PgPool,
    _container: testcontainers::Container<'static, Postgres>,
}

pub async fn setup_test_database() -> TestDb {
    let docker = clients::Cli::default();
    let container = docker.run(Postgres::default());

    let port = container.get_host_port_ipv4(5432);
    let database_url = format!("postgres://postgres:postgres@localhost:{}/postgres", port);

    // Set database URL for SQLx
    env::set_var("DATABASE_URL", &database_url);

    // Connect to the database
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Optionally run migrations
    sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");

    TestDb {
        pool,
        _container: container,
    }
}
