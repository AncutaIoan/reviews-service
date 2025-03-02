use std::fmt::{Display, Formatter};
use crate::models::user::User;
use crate::repository::app_state::AppState;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpResponse, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use serde::Deserialize;
use crate::controller::user_controller::UserError::{FailedToCreate, UserNotFound};

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    username: String,
    email: String,
    password: String,
    phone_number: String
}

#[derive(Debug)]
enum UserError {
    UserNotFound,
    FailedToCreate
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }

}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserNotFound => StatusCode::NOT_FOUND,
            FailedToCreate => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[post("/user/add")]
pub async fn add_user(repo: Data<AppState>, create_user_request: Json<CreateUserRequest>) -> Result<Json<User>, UserError> {
    let new_user = User::new(
        create_user_request.name.to_string(),
        create_user_request.username.to_string(),
        create_user_request.email.to_string(),
        create_user_request.password.to_string(),
        create_user_request.phone_number.to_string(),
        "2025-02-20".to_string(),
        "2025-02-20".to_string()
    );

    match repo.user_repo.create_user(&new_user).await {
        Ok(inserted_review) => Ok(Json(inserted_review)),
        Err(_) => Err(FailedToCreate)
    }
}

#[get("/user/find_by_id/{user_id}")]
pub async fn get_user_by_id(repo: Data<AppState>, user_id: Path<i32>) -> Result<Json<User>, UserError> {
    match repo.user_repo.find_by_id(*user_id).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(UserNotFound)
    }
}