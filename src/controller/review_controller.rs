use crate::models::review::{EntityType, Review};
use std::fmt::{Debug, Display, Formatter};

use actix_web::{error::ResponseError, get, http::{header::ContentType, StatusCode}, post, web::Json, web::Path, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubmitReviewRequest {
    product_id: String,
    added_by: String,
    rating: u8,
}

#[derive(Debug)]
pub enum ReviewError {
    ReviewNotFound,
    FailedToCreate,
    BadReview
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
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[get("/review/{review_id}")]
pub async fn review(review_id: Path<u8>) -> Result<Json<Review>, ReviewError> {
    Ok(Json(Review::new(
        "JohnDoe".to_string(),
        "2025-02-20".to_string(),
        5,
        EntityType::Company,
        "123".to_string()
    )))
}

#[post("/add_review")]
pub async fn add_review(review_request: Json<SubmitReviewRequest>, ) -> Result<Json<Review>, ReviewError> {
    Ok(Json(Review::new(
        review_request.added_by.to_string(),
        "2025-02-20".to_string(),
        review_request.rating,
        EntityType::Person,
        "124".to_string()
    )))
}
