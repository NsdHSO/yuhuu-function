use super::service::ProfileService;
use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use models::internal::{CreateProfileRequest, UpdateProfileRequest};

/// POST /v1/users/:id/profile
/// Create user profile
pub async fn create_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    body: web::Json<CreateProfileRequest>,
    _user: Subject,
) -> Result<HttpResponse> {
    let profile = ProfileService::create_profile(&db, *user_id, body.into_inner()).await?;

    Ok(HttpResponse::Created().json(profile))
}

/// PUT /v1/users/:id/profile
/// Update user profile
pub async fn update_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    body: web::Json<UpdateProfileRequest>,
    _user: Subject,
) -> Result<HttpResponse> {
    let profile = ProfileService::update_profile(&db, *user_id, body.into_inner()).await?;

    Ok(HttpResponse::Ok().json(profile))
}

/// GET /v1/users/:id/profile
/// Get user profile
pub async fn get_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    _user: Subject,
) -> Result<HttpResponse> {
    println!("{:?}", user_id);
    let profile = ProfileService::get_profile(&db, *user_id).await?;

    Ok(HttpResponse::Ok().json(profile))
}
