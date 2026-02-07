use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use chrono::NaiveDate;
use models::internal::{CreateProfileRequest, ProfileResponse, UpdateProfileRequest};
use models::dto::{user_profile, UserProfile};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;

/// POST /v1/users/:id/profile
/// Create user profile
pub async fn create_profile(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i64>,
    body: web::Json<CreateProfileRequest>,
    _user: Subject,
) -> Result<HttpResponse> {
    // Check if profile already exists
    let existing = UserProfile::find()
        .filter(user_profile::Column::UserId.eq(*user_id))
        .one(&**db)
        .await;

    if let Ok(Some(_)) = existing {
        return Ok(HttpResponse::Conflict().json(json!({
            "error": "Profile already exists for this user"
        })));
    }

    // Parse date of birth if provided
    let date_of_birth = match &body.date_of_birth {
        Some(date_str) => match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Invalid date_of_birth format. Use YYYY-MM-DD"
                })))
            }
        },
        None => None,
    };

    let now = chrono::Utc::now().naive_utc();
    let new_profile = user_profile::ActiveModel {
        id: Set(Default::default()),
        uuid: Set(uuid::Uuid::new_v4()),
        user_id: Set(*user_id),
        middle_name: Set(body.middle_name.clone()),
        phone: Set(body.phone.clone()),
        phone_secondary: Set(body.phone_secondary.clone()),
        date_of_birth: Set(date_of_birth),
        gender: Set(body.gender.clone()),
        marital_status: Set(body.marital_status.clone()),
        occupation: Set(body.occupation.clone()),
        nationality: Set(body.nationality.clone()),
        emergency_contact_name: Set(body.emergency_contact_name.clone()),
        emergency_contact_phone: Set(body.emergency_contact_phone.clone()),
        emergency_contact_relationship: Set(body.emergency_contact_relationship.clone()),
        profile_picture_url: Set(body.profile_picture_url.clone()),
        bio: Set(body.bio.clone()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    match new_profile.insert(&**db).await {
        Ok(profile) => {
            let response: ProfileResponse = profile.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// PUT /v1/users/:id/profile
/// Update user profile
pub async fn update_profile(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i64>,
    body: web::Json<UpdateProfileRequest>,
    _user: Subject,
) -> Result<HttpResponse> {
    // Find existing profile
    let profile = UserProfile::find()
        .filter(user_profile::Column::UserId.eq(*user_id))
        .one(&**db)
        .await;

    let existing_profile = match profile {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "error": "Profile not found for this user"
            })))
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            })))
        }
    };

    // Parse date of birth if provided
    let date_of_birth = match &body.date_of_birth {
        Some(date_str) => match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => Some(Some(date)),
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Invalid date_of_birth format. Use YYYY-MM-DD"
                })))
            }
        },
        None => None,
    };

    let mut active_profile: user_profile::ActiveModel = existing_profile.into();

    // Update only provided fields
    if body.middle_name.is_some() {
        active_profile.middle_name = Set(body.middle_name.clone());
    }
    if body.phone.is_some() {
        active_profile.phone = Set(body.phone.clone());
    }
    if body.phone_secondary.is_some() {
        active_profile.phone_secondary = Set(body.phone_secondary.clone());
    }
    if let Some(dob) = date_of_birth {
        active_profile.date_of_birth = Set(dob);
    }
    if body.gender.is_some() {
        active_profile.gender = Set(body.gender.clone());
    }
    if body.marital_status.is_some() {
        active_profile.marital_status = Set(body.marital_status.clone());
    }
    if body.occupation.is_some() {
        active_profile.occupation = Set(body.occupation.clone());
    }
    if body.nationality.is_some() {
        active_profile.nationality = Set(body.nationality.clone());
    }
    if body.emergency_contact_name.is_some() {
        active_profile.emergency_contact_name = Set(body.emergency_contact_name.clone());
    }
    if body.emergency_contact_phone.is_some() {
        active_profile.emergency_contact_phone = Set(body.emergency_contact_phone.clone());
    }
    if body.emergency_contact_relationship.is_some() {
        active_profile.emergency_contact_relationship =
            Set(body.emergency_contact_relationship.clone());
    }
    if body.profile_picture_url.is_some() {
        active_profile.profile_picture_url = Set(body.profile_picture_url.clone());
    }
    if body.bio.is_some() {
        active_profile.bio = Set(body.bio.clone());
    }

    active_profile.updated_at = Set(chrono::Utc::now().naive_utc());

    match active_profile.update(&**db).await {
        Ok(updated) => {
            let response: ProfileResponse = updated.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// GET /v1/users/:id/profile
/// Get user profile
pub async fn get_profile(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i64>,
    _user: Subject,
) -> Result<HttpResponse> {
    match UserProfile::find()
        .filter(user_profile::Column::UserId.eq(*user_id))
        .one(&**db)
        .await
    {
        Ok(Some(profile)) => {
            let response: ProfileResponse = profile.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "Profile not found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Configure profile routes
pub fn configure_profiles(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}/profile")
            .route(web::post().to(create_profile))
            .route(web::put().to(update_profile))
            .route(web::get().to(get_profile)),
    );
}
