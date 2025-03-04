use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) password_confirmation: String,
    pub(crate) phone_number: String
}