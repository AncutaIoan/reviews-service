use std::sync::Arc;
use sqlx::PgPool;

use crate::repository::repository::PostgresRepository;
use crate::repository::review_repository::ReviewRepository;
use crate::repository::user_repository::UserRepository;

pub struct AppState {
    pub review_repo: Arc<dyn ReviewRepository + Send + Sync>,
    pub user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let review_repo = Arc::new(PostgresRepository::new(pool.clone())) as Arc<dyn ReviewRepository + Send + Sync>;
        let user_repo = Arc::new(PostgresRepository::new(pool.clone())) as Arc<dyn UserRepository + Send + Sync>;
        AppState {
            review_repo,
            user_repo
        }
    }
}
