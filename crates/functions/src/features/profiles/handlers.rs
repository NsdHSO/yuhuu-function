use super::service::ProfileService;
use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use models::internal::{CreateProfileRequest, UpdateProfileRequest};
use crate::features::users::service::UserService;
use http_response::{create_response, HttpCodeW};

/// POST /v1/users/:id/profile
/// Create user profile
pub async fn create_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    body: web::Json<CreateProfileRequest>,
    _user: Subject,
) -> Result<HttpResponse> {
    let profile = ProfileService::create_profile(&db, *user_id, body.into_inner()).await?;

    let resp = create_response(profile, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
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

    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /v1/users/:id/profile
/// Get user profile
pub async fn get_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    _user: Subject,
) -> Result<HttpResponse> {
    let profile = ProfileService::get_profile(&db, *user_id).await?;

    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /v1/me/profile
/// Get the authenticated user's profile (no numeric id)
pub async fn get_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let me = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let profile = ProfileService::get_profile(&db, me.id).await?;
    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// POST /v1/me/profile
/// Create the authenticated user's profile (no numeric id)
pub async fn create_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
    body: web::Json<CreateProfileRequest>,
) -> Result<HttpResponse> {
    let me = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let created = ProfileService::create_profile(&db, me.id, body.into_inner()).await?;
    let resp = create_response(created, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

/// PUT /v1/me/profile
/// Update the authenticated user's profile (no numeric id)
pub async fn update_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
    body: web::Json<UpdateProfileRequest>,
) -> Result<HttpResponse> {
    let me = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let updated = ProfileService::update_profile(&db, me.id, body.into_inner()).await?;
    let resp = create_response(updated, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
