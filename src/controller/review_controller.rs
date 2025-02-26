use crate::models::review::{EntityType, Review};
use std::fmt::{Debug, Display, Formatter};

use actix_web::{error::ResponseError, get, http::{header::ContentType, StatusCode}, post, web::Json, web::Path, HttpResponse};
use actix_web::web::Data;
use serde::Deserialize;
use sqlx::{Error, PgPool};
use crate::controller::review_controller::ReviewError::{FailedToCreate, FailedToRetrieve, ReviewNotFound};

#[derive(Deserialize)]
pub struct SubmitReviewRequest {
    product_id: String,
    added_by: String,
    rating: i32,
}

#[derive(Debug)]
pub enum ReviewError {
    ReviewNotFound,
    FailedToCreate,
    BadReview,
    FailedToRetrieve
}

impl Display for ReviewError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ReviewError {
    fn status_code(&self) -> StatusCode {
        match self {
            ReviewError::ReviewNotFound => StatusCode::NOT_FOUND,
            ReviewError::FailedToCreate => StatusCode::INTERNAL_SERVER_ERROR,
            ReviewError::BadReview => StatusCode::BAD_REQUEST,
            ReviewError::FailedToRetrieve => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[post("/review/add")]
pub async fn add_review(pool: Data<PgPool>, review_request: Json<SubmitReviewRequest>, ) -> Result<Json<Review>, ReviewError> {
    let new_review = Review::new(
        review_request.added_by.to_string(),
        "2025-02-20".to_string(),
        review_request.rating,
        EntityType::Person,
        "124".to_string()
    );

    match create_review(&pool, &new_review).await {
        Ok(inserted_review) => Ok(Json(inserted_review)),
        Err(_) => Err(FailedToCreate)
    }
}

#[get("/review/find_by_id/{review_id}")]
pub async fn get_review(pool: Data<PgPool>, review_id: Path<i32>, ) -> Result<Json<Review>, ReviewError> {
    match find_review_by_id(&pool, *review_id).await {
        Ok(review) => Ok(Json(review)),
        Err(_) => Err(ReviewNotFound)
    }
}

#[get("/review/get_all_reviews")]
pub async fn get_all_reviews(pool: Data<PgPool>) -> Result<Json<Vec<Review>>, ReviewError> {
    match find_all_reviews(&pool).await {
        Ok(reviews) => Ok(Json(reviews)),
        Err(_) => Err(FailedToRetrieve)
    }
}

async fn create_review(pool: &PgPool, review: &Review) -> Result<Review, Error> {
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
        .fetch_one(pool)
        .await
    {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Error inserting review: {}", e);
            Err(e)
        }
    }

}


async fn find_review_by_id(pool: &PgPool, id: i32) -> Result<Review, Error> {
    let query = r#"
        SELECT id, added_by, added_at, rating, entity_type, entity_id
        FROM reviews
        WHERE id = $1
    "#;

    match sqlx::query_as::<_, Review>(query)
        .bind(&id)
        .fetch_one(pool)
        .await {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Error retrieving review: {}", e);
            Err(e)
        }
    }
}

async fn find_all_reviews(pool: &PgPool) -> Result<Vec<Review>, Error> {
    let query = r#"
        SELECT id, added_by, added_at, rating, entity_type, entity_id
        FROM reviews
    "#;

    match sqlx::query_as::<_, Review>(query)
        .fetch_all(pool)
        .await {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Error retrieving reviews: {}", e);
            Err(e)
        }
    }
}
