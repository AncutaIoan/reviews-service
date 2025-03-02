use sqlx::Error;
use crate::models::user::User;
use crate::repository::repository::PostgresRepository;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<User, Error>;
    async fn find_by_id(&self, id: i32) -> Result<User, Error>;
}


#[async_trait::async_trait]
impl UserRepository for PostgresRepository {
    async fn create_user(&self, user: &User) -> Result<User, Error> {
        let query = r#"
                           INSERT INTO users (name, username, email, password, phone_number, created_at, updated_at)
                           VALUES ($1, $2, $3, $4, $5, $6, $7)
                           RETURNING *
                         "#;

        match sqlx::query_as::<_, User>(query)
            .bind(&user.name)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.phone_number)
            .bind(&user.created_at)
            .bind(&user.updated_at)
            .fetch_one(&self.pool)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error inserting user: {}", e);
                Err(e)
            }
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<User, Error> {
        let query = r#"
                            SELECT *
                            FROM users
                            WHERE id = $1
                          "#;

        match sqlx::query_as::<_, User>(query)
            .bind(&id).fetch_one(&self.pool)
            .await {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error retrieving user: {}", e);
                Err(e)
            }
        }
    }
}