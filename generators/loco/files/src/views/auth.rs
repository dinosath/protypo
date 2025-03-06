use serde::{Deserialize, Serialize};

use crate::models::user::Model;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            email: user.email.clone(),
        }
    }
}
