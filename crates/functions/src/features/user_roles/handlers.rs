use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use models::internal::AssignRoleRequest;

use super::service::UserRoleService;

/// POST /v1/users/:user_id/roles
/// Assign a role to a user (Admin only)
pub async fn assign_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    body: web::Json<AssignRoleRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    // Get the church user ID from auth_user_id (subject.sub)
    // For now, we'll use a placeholder - you should implement get_user_by_auth_id
    // TODO: Get assigned_by user's church ID from subject.sub
    let assigned_by = 1; // Placeholder

    let user_role = UserRoleService::assign_role(&db, user_id, body.role_id, assigned_by).await?;

    Ok(HttpResponse::Created().json(user_role))
}

/// DELETE /v1/users/:user_id/roles/:role_id
/// Remove a role from a user (Admin only)
pub async fn remove_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let (user_id, role_id) = path.into_inner();

    UserRoleService::remove_role(&db, user_id, role_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// GET /v1/users/:user_id/roles
/// Get all roles for a specific user
pub async fn get_user_roles(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    let roles = UserRoleService::get_user_roles(&db, user_id).await?;

    Ok(HttpResponse::Ok().json(roles))
}

/// GET /v1/roles/:role_id/users
/// Get all users with a specific role
pub async fn get_users_by_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let role_id = path.into_inner();

    let users = UserRoleService::get_users_by_role(&db, role_id).await?;

    Ok(HttpResponse::Ok().json(users))
}
