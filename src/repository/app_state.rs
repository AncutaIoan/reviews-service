use crate::repository::review_repository::{PostgresRepository, ReviewRepository};

use std::sync::Arc;
use sqlx::PgPool;

pub struct AppState {
    pub review_repo: Arc<dyn ReviewRepository + Send + Sync>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let review_repo = Arc::new(PostgresRepository::new(pool.clone())) as Arc<dyn ReviewRepository + Send + Sync>;

        AppState {
            review_repo,
        }
    }
}
