use chrono::NaiveDate;
use http_response::{CustomError, HttpCodeW};
use models::dto::{user_profile, UserProfile};
use models::internal::{CreateProfileRequest, ProfileResponse, UpdateProfileRequest};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use user_profile::Column::UserId;

pub struct ProfileService;

impl ProfileService {
    /// Create a new user profile
    pub async fn create_profile(
        db: &DatabaseConnection,
        user_id: i64,
        request: CreateProfileRequest,
    ) -> Result<ProfileResponse, CustomError> {
        // Check if profile already exists
        let existing = UserProfile::find()
            .filter(UserId.eq(user_id))
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(CustomError::new(
                HttpCodeW::Conflict,
                "Profile already exists for this user".to_string(),
            ));
        }

        // Parse date of birth if provided
        let date_of_birth = Self::parse_date(&request.date_of_birth)?;

        let now = chrono::Utc::now().naive_utc();
        let new_profile = user_profile::ActiveModel {
            id: Set(Default::default()),
            uuid: Set(uuid::Uuid::new_v4()),
            user_id: Set(user_id),
            middle_name: Set(request.middle_name),
            phone: Set(request.phone),
            phone_secondary: Set(request.phone_secondary),
            date_of_birth: Set(date_of_birth),
            gender: Set(request.gender),
            marital_status: Set(request.marital_status),
            occupation: Set(request.occupation),
            nationality: Set(request.nationality),
            emergency_contact_name: Set(request.emergency_contact_name),
            emergency_contact_phone: Set(request.emergency_contact_phone),
            emergency_contact_relationship: Set(request.emergency_contact_relationship),
            profile_picture_url: Set(request.profile_picture_url),
            bio: Set(request.bio),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let profile = new_profile.insert(db).await?;

        Ok(profile.into())
    }

    /// Update an existing user profile
    pub async fn update_profile(
        db: &DatabaseConnection,
        user_id: i64,
        request: UpdateProfileRequest,
    ) -> Result<ProfileResponse, CustomError> {
        // Find existing profile
        let existing_profile = UserProfile::find()
            .filter(UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "Profile not found for this user".to_string(),
                )
            })?;

        let mut active_profile: user_profile::ActiveModel = existing_profile.into();

        // Update only provided fields
        if request.middle_name.is_some() {
            active_profile.middle_name = Set(request.middle_name);
        }
        if request.phone.is_some() {
            active_profile.phone = Set(request.phone);
        }
        if request.phone_secondary.is_some() {
            active_profile.phone_secondary = Set(request.phone_secondary);
        }
        if let Some(dob) = Self::parse_date(&request.date_of_birth)? {
            active_profile.date_of_birth = Set(Some(dob));
        }
        if request.gender.is_some() {
            active_profile.gender = Set(request.gender);
        }
        if request.marital_status.is_some() {
            active_profile.marital_status = Set(request.marital_status);
        }
        if request.occupation.is_some() {
            active_profile.occupation = Set(request.occupation);
        }
        if request.nationality.is_some() {
            active_profile.nationality = Set(request.nationality);
        }
        if request.emergency_contact_name.is_some() {
            active_profile.emergency_contact_name = Set(request.emergency_contact_name);
        }
        if request.emergency_contact_phone.is_some() {
            active_profile.emergency_contact_phone = Set(request.emergency_contact_phone);
        }
        if request.emergency_contact_relationship.is_some() {
            active_profile.emergency_contact_relationship =
                Set(request.emergency_contact_relationship);
        }
        if request.profile_picture_url.is_some() {
            active_profile.profile_picture_url = Set(request.profile_picture_url);
        }
        if request.bio.is_some() {
            active_profile.bio = Set(request.bio);
        }

        active_profile.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated = active_profile.update(db).await?;

        Ok(updated.into())
    }

    /// Get a user profile by user ID
    pub async fn get_profile(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<ProfileResponse, CustomError> {
        let profile = UserProfile::find()
            .filter(UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "Profile not found for this user".to_string(),
                )
            })?;

        Ok(profile.into())
    }

    /// Parse date string to NaiveDate
    fn parse_date(date_str: &Option<String>) -> Result<Option<NaiveDate>, CustomError> {
        match date_str {
            Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map(Some)
                .map_err(|_| {
                    CustomError::new(
                        HttpCodeW::BadRequest,
                        "Invalid date_of_birth format. Use YYYY-MM-DD".to_string(),
                    )
                }),
            None => Ok(None),
        }
    }
}
