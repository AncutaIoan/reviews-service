use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

const EXPIRATION_TIME: usize = 3600;

pub fn create_jwt(user_id: i32) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: user_id.to_string(),
        exp: chrono::Utc::now().timestamp() as usize + EXPIRATION_TIME,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("JWT token creation failed")
}

pub fn verify_jwt(token: &str) -> Option<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
        .map(|data| data.claims)
        .ok()
}