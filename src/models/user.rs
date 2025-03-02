use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub(crate) struct User{
    id: i32,
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) phone_number: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String
}

impl User {
    pub(crate) fn new(name: String, username: String, email: String, password: String, phone_number: String, created_at: String, updated_at: String) -> Self {
        User{
            id: 0,
            name,
            username,
            email,
            password,
            phone_number,
            created_at,
            updated_at
        }
    }
}