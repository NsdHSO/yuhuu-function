use serde::{Deserialize, Serialize};

use crate::internal::{CreateProfileRequest, ProfileResponse, UserResponse};

#[derive(Debug, Deserialize)]
pub struct BootstrapRequest {
    #[serde(default)]
    pub create_profile_if_missing: bool,
    #[serde(default)]
    pub profile: Option<CreateProfileRequest>,
}

#[derive(Debug, Serialize)]
pub struct BootstrapResponse {
    pub user: UserResponse,
    pub profile: Option<ProfileResponse>,
    pub created: BootstrapCreated,
}

#[derive(Debug, Serialize, Default)]
pub struct BootstrapCreated {
    pub linked: bool,
    pub profile: bool,
}