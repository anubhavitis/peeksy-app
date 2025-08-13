use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<String>,
    pub user_id: String,
    pub user_email: String,
    pub stored_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthValidation {
    pub is_valid: bool,
    pub is_expired: bool,
    pub is_near_expiry: bool,
    pub expires_in_seconds: Option<i64>,
    pub reason: Option<String>,
}
