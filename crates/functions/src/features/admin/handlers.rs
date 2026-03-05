use actix_web::{web, HttpResponse, Result};
use auth_integration::AdminGuard;
use http_response::{create_response, HttpCodeW};
use models::internal::SearchUsersQuery;

use super::super::family_relationships::service::FamilyRelationshipService;
use super::super::membership_history::service::MembershipHistoryService;
use super::super::spiritual_milestones::service::SpiritualMilestoneService;
use super::super::user_skills::service::UserSkillService;
use super::service::AdminService;

/// Search users by name (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/search?q={searchTerm}
///
/// # Query Parameters
/// - `q`: Search term for middle_name (min 2 characters)
///
/// # Authorization
/// Requires Admin role (enforced by AdminGuard)
///
/// # Returns
/// - 200 OK: List of matching user profiles
/// - 404 Not Found: No users found matching search term
/// - 400 Bad Request: Invalid search term (less than 2 characters)
/// - 403 Forbidden: User is not an admin
/// - 401 Unauthorized: Missing or invalid JWT token
pub async fn search_users(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<SearchUsersQuery>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let results = AdminService::search_users_by_name(&db, &query.q).await?;

    if results.is_empty() {
        let resp = create_response(results, HttpCodeW::NotFound);
        return Ok(HttpResponse::NotFound().json(resp));
    }

    let resp = create_response(results, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Get milestones for a specific user (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/{user_id}/milestones
///
/// # Path Parameters
/// - `user_id`: The ID of the user whose milestones to retrieve
///
/// # Authorization
/// Requires Admin role (enforced by AdminGuard)
///
/// # Returns
/// - 200 OK: List of user's spiritual milestones
/// - 403 Forbidden: User is not an admin
/// - 401 Unauthorized: Missing or invalid JWT token
pub async fn get_user_milestones(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let results = SpiritualMilestoneService::list_by_user(&db, user_id.into_inner()).await?;

    let resp = create_response(results, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Get skills for a specific user (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/{user_id}/skills
///
/// # Path Parameters
/// - `user_id`: The ID of the user whose skills to retrieve
///
/// # Authorization
/// Requires Admin role (enforced by AdminGuard)
///
/// # Returns
/// - 200 OK: List of user's skills
/// - 403 Forbidden: User is not an admin
/// - 401 Unauthorized: Missing or invalid JWT token
pub async fn get_user_skills(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let results = UserSkillService::list_by_user(&db, user_id.into_inner()).await?;

    let resp = create_response(results, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Get family relationships for a specific user (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/{user_id}/family
///
/// # Path Parameters
/// - `user_id`: The ID of the user whose family relationships to retrieve
///
/// # Authorization
/// Requires Admin role (enforced by AdminGuard)
///
/// # Returns
/// - 200 OK: List of user's family relationships
/// - 403 Forbidden: User is not an admin
/// - 401 Unauthorized: Missing or invalid JWT token
pub async fn get_user_family(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let results = FamilyRelationshipService::list_by_user(&db, user_id.into_inner()).await?;

    let resp = create_response(results, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Get membership history for a specific user (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/{user_id}/membership-history
///
/// # Path Parameters
/// - `user_id`: The ID of the user whose membership history to retrieve
///
/// # Authorization
/// Requires Admin role (enforced by AdminGuard)
///
/// # Returns
/// - 200 OK: List of user's membership history entries
/// - 403 Forbidden: User is not an admin
/// - 401 Unauthorized: Missing or invalid JWT token
pub async fn get_user_membership_history(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let results = MembershipHistoryService::list_by_user(&db, user_id.into_inner()).await?;

    let resp = create_response(results, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
