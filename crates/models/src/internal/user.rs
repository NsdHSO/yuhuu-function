use serde::{Deserialize, Serialize};

/// Request to link an auth server user to church system
#[derive(Debug, Deserialize)]
pub struct LinkUserRequest {
    pub email: String,
}

/// Query parameters for listing users
#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

/// Response for user data (combines auth + church data)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub auth_user_id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub is_email_verified: bool,
    pub created_at: String,
    pub updated_at: String,
}
