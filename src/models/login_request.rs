use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}
