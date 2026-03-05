use actix_web::{web, HttpResponse, Result};
use auth_integration::AdminGuard;
use http_response::{create_response, HttpCodeW};
use models::internal::SearchUsersQuery;

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
