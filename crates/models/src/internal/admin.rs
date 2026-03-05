use serde::{Deserialize, Serialize};

use crate::dto::UserProfileModel;

/// Query parameters for user search
#[derive(Debug, Deserialize)]
pub struct SearchUsersQuery {
    /// Search term for middle_name (min 2 chars)
    pub q: String,
}

/// User search result response
#[derive(Debug, Serialize)]
pub struct UserSearchResult {
    /// Profile ID
    pub id: i64,

    /// User ID (references users table)
    /// CRITICAL: Frontend uses this to fetch family/milestones/membership/skills
    pub user_id: i64,

    /// User's middle name
    pub middle_name: Option<String>,

    /// User's phone number
    pub phone: Option<String>,

    /// Profile picture URL
    pub profile_picture_url: Option<String>,
}

impl From<UserProfileModel> for UserSearchResult {
    fn from(profile: UserProfileModel) -> Self {
        UserSearchResult {
            id: profile.id,
            user_id: profile.user_id,
            middle_name: profile.middle_name,
            phone: profile.phone,
            profile_picture_url: profile.profile_picture_url,
        }
    }
}
