use crate::controller::user_controller::UserError::{FailedToCreate, UserNotFound};
use crate::models::login_request::LoginRequest;
use crate::models::register_user_request::RegisterRequest;
use crate::models::user::User;
use crate::repository::app_state::AppState;
use crate::service::jws::{create_jwt, verify_jwt};
use actix_web::cookie::Cookie;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpRequest, HttpResponse, Responder, ResponseError};
use bcrypt::{hash, verify};
use serde_json::json;
use std::fmt::{Display, Formatter};

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

#[post("/user/register")]
pub async fn register_user(repo: Data<AppState>, register_user: Json<RegisterRequest>) -> impl Responder {
    let hashed_password = hash(&register_user.password, 10).unwrap();

    let new_user = User::new(
        register_user.name.to_string(),
        register_user.username.to_string(),
        register_user.email.to_string(),
        hashed_password.to_string(),
        register_user.phone_number.to_string()
    );

    match repo.user_repo.create_user(&new_user).await {
        Ok(_) => HttpResponse::Created().json(json!({"message": "User registered"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Registration failed"})),
    }
}

#[post("/user/login")]
pub async fn login(repo: Data<AppState>, login_request: Json<LoginRequest>) -> impl Responder {
    match repo.user_repo.find_by_email(login_request.email.to_string()).await {
        Ok(user) => {
            if verify(&login_request.password, &user.password).unwrap() {
                let token = create_jwt(user.id);

                let cookie = Cookie::new("auth_token", token.clone());

                HttpResponse::Ok()
                    .cookie(cookie)
                    .json(json!({"message": "Login successful", "token": token}))        }
            else {
                HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"}))
            }
        } ,
        Err(_) => HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"})),
    }
}

#[get("/user/me")]
pub async fn me(repo: Data<AppState>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("auth_token") {
        if let Some(claims) = verify_jwt(cookie.value()) {
            return match repo.user_repo.find_by_id(claims.sub.parse::<i32>().unwrap()).await {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
            }
        }
    }

    HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
}

#[get("/user/find_by_id/{user_id}")]
pub async fn get_user_by_id(repo: Data<AppState>, user_id: Path<i32>) -> Result<Json<User>, UserError> {
    match repo.user_repo.find_by_id(*user_id).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(UserNotFound)
    }
}