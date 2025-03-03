use crate::controller::review_controller::ReviewError::{FailedToCreate, FailedToRetrieve, ReviewNotFound};
use crate::models::review::{EntityType, Review};
use crate::repository::app_state::AppState;
use actix_web::web::Data;
use actix_web::{delete, error::ResponseError, get, http::{header::ContentType, StatusCode}, post, web::Json, web::Path, HttpResponse};
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};

#[derive(Deserialize)]
pub struct SubmitReviewRequest {
    product_id: String,
    added_by: String,
    rating: i32
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
pub async fn add_review(repo: Data<AppState>, review_request: Json<SubmitReviewRequest>) -> Result<Json<Review>, ReviewError> {
    let new_review = Review::new(
        review_request.added_by.to_string(),
        review_request.rating,
        EntityType::Person,
        "124".to_string()
    );

    match repo.review_repo.create_review(&new_review).await {
        Ok(inserted_review) => Ok(Json(inserted_review)),
        Err(_) => Err(FailedToCreate)
    }
}

#[get("/review/find_by_id/{review_id}")]
pub async fn get_review(repo: Data<AppState>, review_id: Path<i32>) -> Result<Json<Review>, ReviewError> {
    match repo.review_repo.find_review_by_id(*review_id).await {
        Ok(review) => Ok(Json(review)),
        Err(_) => Err(ReviewNotFound)
    }
}

#[delete("/review/delete/{review_id}")]
pub async fn delete_review(repo: Data<AppState>, review_id: Path<i32>) -> Result<Json<u64>, ReviewError> {
    match repo.review_repo.delete_review_by_id(*review_id).await {
        Ok(number_of_affected_rows) => Ok(Json(number_of_affected_rows)),
        Err(_) => Err(ReviewNotFound)
    }
}

#[get("/review/get_all_reviews")]
pub async fn get_all_reviews(repo: Data<AppState>) -> Result<Json<Vec<Review>>, ReviewError> {
    match repo.review_repo.find_all_reviews().await {
        Ok(reviews) => Ok(Json(reviews)),
        Err(_) => Err(FailedToRetrieve)
    }
}