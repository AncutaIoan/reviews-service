use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub(crate) struct User{
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) phone_number: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime
}

impl User {
    pub(crate) fn new(name: String, username: String, email: String, password: String, phone_number: String) -> Self {
        let time = Utc::now().naive_utc();
        let created_at = time;
        let updated_at = time;
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