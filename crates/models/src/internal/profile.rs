use serde::{Deserialize, Serialize};

use crate::dto::UserProfileModel;

/// Request to create a new user profile
#[derive(Debug, Deserialize, Clone)]
pub struct CreateProfileRequest {
    pub middle_name: Option<String>,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,
    pub date_of_birth: Option<String>, // "YYYY-MM-DD" format
    pub gender: Option<String>,
    pub marital_status: Option<String>,
    pub occupation: Option<String>,
    pub nationality: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub emergency_contact_relationship: Option<String>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
}

/// Request to update user profile (partial update)
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub middle_name: Option<String>,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub marital_status: Option<String>,
    pub occupation: Option<String>,
    pub nationality: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub emergency_contact_relationship: Option<String>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
}

/// Response for user profile data
#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: i64,
    pub user_id: i64,
    pub middle_name: Option<String>,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub marital_status: Option<String>,
    pub occupation: Option<String>,
    pub nationality: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub emergency_contact_relationship: Option<String>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
}

impl From<UserProfileModel> for ProfileResponse {
    fn from(profile: UserProfileModel) -> Self {
        ProfileResponse {
            id: profile.id,
            user_id: profile.user_id,
            middle_name: profile.middle_name,
            phone: profile.phone,
            phone_secondary: profile.phone_secondary,
            date_of_birth: profile.date_of_birth.map(|d| d.to_string()),
            gender: profile.gender,
            marital_status: profile.marital_status,
            occupation: profile.occupation,
            nationality: profile.nationality,
            emergency_contact_name: profile.emergency_contact_name,
            emergency_contact_phone: profile.emergency_contact_phone,
            emergency_contact_relationship: profile.emergency_contact_relationship,
            profile_picture_url: profile.profile_picture_url,
            bio: profile.bio,
        }
    }
}
