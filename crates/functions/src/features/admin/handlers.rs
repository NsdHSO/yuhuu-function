use actix_web::{web, HttpResponse, Result};
use auth_integration::AdminGuard;
use http_response::{create_response, HttpCodeW};
use models::internal::{
    CreateFamilyRelationshipRequest, CreateMembershipHistoryRequest,
    CreateSpiritualMilestoneRequest, CreateUserSkillRequest, SearchUsersQuery,
    UpdateFamilyRelationshipRequest, UpdateMembershipHistoryRequest,
    UpdateSpiritualMilestoneRequest, UpdateUserSkillRequest,
};

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

// ============================================================================
// FAMILY RELATIONSHIPS - ADMIN CRUD
// ============================================================================

/// Create family relationship for a user (admin-only)
///
/// # Endpoint
/// POST /v1/admin/users/{user_id}/family
pub async fn create_user_family(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    request: web::Json<CreateFamilyRelationshipRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let result =
        FamilyRelationshipService::create(&db, user_id.into_inner(), request.into_inner()).await?;

    let resp = create_response(result, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

/// Get specific family relationship (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/{user_id}/family/{id}
pub async fn get_user_family_by_id(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, relationship_id) = path.into_inner();
    let result = FamilyRelationshipService::get_by_id(&db, user_id, relationship_id).await?;

    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Update family relationship (admin-only)
///
/// # Endpoint
/// PUT /v1/admin/users/{user_id}/family/{id}
pub async fn update_user_family(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    request: web::Json<UpdateFamilyRelationshipRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, relationship_id) = path.into_inner();
    let result = FamilyRelationshipService::update(
        &db,
        user_id,
        relationship_id,
        request.into_inner(),
    )
    .await?;

    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Delete family relationship (admin-only)
///
/// # Endpoint
/// DELETE /v1/admin/users/{user_id}/family/{id}
pub async fn delete_user_family(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, relationship_id) = path.into_inner();
    FamilyRelationshipService::delete(&db, user_id, relationship_id).await?;

    let resp = create_response("Family relationship deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

// ============================================================================
// SPIRITUAL MILESTONES - ADMIN CRUD
// ============================================================================

/// Create spiritual milestone for a user (admin-only)
///
/// # Endpoint
/// POST /v1/admin/users/{user_id}/milestones
pub async fn create_user_milestone(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    request: web::Json<CreateSpiritualMilestoneRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let result =
        SpiritualMilestoneService::create(&db, user_id.into_inner(), request.into_inner()).await?;

    let resp = create_response(result, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

/// Get specific spiritual milestone (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/{user_id}/milestones/{id}
pub async fn get_user_milestone_by_id(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, milestone_id) = path.into_inner();
    let result = SpiritualMilestoneService::get_by_id(&db, user_id, milestone_id).await?;

    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Update spiritual milestone (admin-only)
///
/// # Endpoint
/// PUT /v1/admin/users/{user_id}/milestones/{id}
pub async fn update_user_milestone(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    request: web::Json<UpdateSpiritualMilestoneRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, milestone_id) = path.into_inner();
    let result =
        SpiritualMilestoneService::update(&db, user_id, milestone_id, request.into_inner())
            .await?;

    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Delete spiritual milestone (admin-only)
///
/// # Endpoint
/// DELETE /v1/admin/users/{user_id}/milestones/{id}
pub async fn delete_user_milestone(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, milestone_id) = path.into_inner();
    SpiritualMilestoneService::delete(&db, user_id, milestone_id).await?;

    let resp = create_response("Spiritual milestone deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

// ============================================================================
// MEMBERSHIP HISTORY - ADMIN CRUD
// ============================================================================

/// Create membership history entry for a user (admin-only)
///
/// # Endpoint
/// POST /v1/admin/users/{user_id}/membership-history
pub async fn create_user_membership_history(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    request: web::Json<CreateMembershipHistoryRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let result =
        MembershipHistoryService::create(&db, user_id.into_inner(), request.into_inner()).await?;

    let resp = create_response(result, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

/// Get specific membership history entry (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/{user_id}/membership-history/{id}
pub async fn get_user_membership_history_by_id(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, history_id) = path.into_inner();
    let result = MembershipHistoryService::get_by_id(&db, user_id, history_id).await?;

    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Update membership history entry (admin-only)
///
/// # Endpoint
/// PUT /v1/admin/users/{user_id}/membership-history/{id}
pub async fn update_user_membership_history(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    request: web::Json<UpdateMembershipHistoryRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, history_id) = path.into_inner();
    let result =
        MembershipHistoryService::update(&db, user_id, history_id, request.into_inner()).await?;

    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Delete membership history entry (admin-only)
///
/// # Endpoint
/// DELETE /v1/admin/users/{user_id}/membership-history/{id}
pub async fn delete_user_membership_history(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, history_id) = path.into_inner();
    MembershipHistoryService::delete(&db, user_id, history_id).await?;

    let resp = create_response("Membership history entry deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

// ============================================================================
// USER SKILLS - ADMIN CRUD
// ============================================================================

/// Create skill for a user (admin-only)
///
/// # Endpoint
/// POST /v1/admin/users/{user_id}/skills
pub async fn create_user_skill(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_id: web::Path<i64>,
    request: web::Json<CreateUserSkillRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let result =
        UserSkillService::create(&db, user_id.into_inner(), request.into_inner()).await?;

    let resp = create_response(result, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

/// Get specific skill (admin-only)
///
/// # Endpoint
/// GET /v1/admin/users/{user_id}/skills/{id}
pub async fn get_user_skill_by_id(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, skill_id) = path.into_inner();
    let result = UserSkillService::get_by_id(&db, user_id, skill_id).await?;

    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Update skill (admin-only)
///
/// # Endpoint
/// PUT /v1/admin/users/{user_id}/skills/{id}
pub async fn update_user_skill(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    request: web::Json<UpdateUserSkillRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, skill_id) = path.into_inner();
    let result = UserSkillService::update(&db, user_id, skill_id, request.into_inner()).await?;

    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Delete skill (admin-only)
///
/// # Endpoint
/// DELETE /v1/admin/users/{user_id}/skills/{id}
pub async fn delete_user_skill(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let (user_id, skill_id) = path.into_inner();
    UserSkillService::delete(&db, user_id, skill_id).await?;

    let resp = create_response("Skill deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
