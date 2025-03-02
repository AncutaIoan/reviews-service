use sqlx::{Error, PgPool};
use crate::models::review::Review;

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn create_review(&self, review: &Review) -> Result<Review, Error>;
    async fn find_review_by_id(&self, id: i32) -> Result<Review, Error>;
    async fn delete_review_by_id(&self, id: i32) -> Result<u64, Error>;
    async fn find_all_reviews(&self) -> Result<Vec<Review>, Error>;
}


#[async_trait::async_trait]
impl ReviewRepository for PostgresRepository {
    async fn create_review(&self, review: &Review) -> Result<Review,  Error> {
        let query = r#"
                            INSERT INTO reviews (added_by, added_at, rating, entity_type, entity_id)
                            VALUES ($1, $2, $3, $4, $5)
                            RETURNING id, added_by, added_at, rating, entity_type, entity_id
                        "#;

        match sqlx::query_as::<_, Review>(query)
            .bind(&review.added_by)
            .bind(&review.added_at)
            .bind(review.rating)
            .bind(&review.entity_type)
            .bind(&review.entity_id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error inserting review: {}", e);
                Err(e)
            }
        }

    }


    async fn find_review_by_id(&self, id: i32) -> Result<Review, Error> {
        let query = r#"
                            SELECT id, added_by, added_at, rating, entity_type, entity_id
                            FROM reviews
                            WHERE id = $1
                        "#;

        match sqlx::query_as::<_, Review>(query)
            .bind(&id)
            .fetch_one(&self.pool)
            .await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error retrieving review: {}", e);
                Err(e)
            }
        }
    }

    async fn delete_review_by_id(&self, id: i32) -> Result<u64, Error> {
        let query = r#"
                            DELETE FROM reviews
                            WHERE id = $1
                        "#;

        match sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await {
            Ok(result) => Ok(result.rows_affected()), // Return number of affected rows
            Err(e) => {
                eprintln!("Error deleting review: {}", e);
                Err(e)
            }
        }
    }

    async fn find_all_reviews(&self) -> Result<Vec<Review>, Error> {
        let query = r#"
                            SELECT id, added_by, added_at, rating, entity_type, entity_id
                            FROM reviews
                        "#;

        match sqlx::query_as::<_, Review>(query)
            .fetch_all(&self.pool)
            .await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error retrieving reviews: {}", e);
                Err(e)
            }
        }
    }
}