use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use models::internal::ListUsersQuery;

use super::service::UserService;

/// POST /v1/users/link
/// Link the authenticated user from auth server to church system
pub async fn link_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let response = UserService::link_user(&db, &subject.sub).await?;

    if response.message.contains("already") {
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::Created().json(response))
    }
}

/// GET /v1/users/:id
/// Get a specific church user by ID
pub async fn get_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let user = UserService::get_user_by_id(&db, user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}

/// GET /v1/users
/// List all church users (paginated)
pub async fn list_users(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListUsersQuery>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let response = UserService::list_users(&db, query.page, query.limit).await?;

    Ok(HttpResponse::Ok().json(response))
}
