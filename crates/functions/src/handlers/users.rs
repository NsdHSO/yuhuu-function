use actix_web::{web, HttpResponse, Result};
use auth_integration::{UserContext, UserService};
use models::internal::{LinkUserRequest, ListUsersQuery, UserResponse};
use serde_json::json;

/// POST /v1/users/link
/// Link an auth server user to the church system
pub async fn link_user(
    user_service: web::Data<UserService>,
    body: web::Json<LinkUserRequest>,
    _user: UserContext, // Requires authentication
) -> Result<HttpResponse> {
    match user_service.link_auth_user_by_email(&body.email).await {
        Ok(church_user) => Ok(HttpResponse::Ok().json(json!({
            "id": church_user.id,
            "auth_user_id": church_user.auth_user_id,
            "created_at": church_user.created_at.to_string(),
            "updated_at": church_user.updated_at.to_string(),
            "message": "User linked successfully"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// GET /v1/users/:id
/// Get complete user (auth server data + church data)
pub async fn get_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<i64>,
    _user: UserContext, // Requires authentication
) -> Result<HttpResponse> {
    match user_service.get_complete_user(*user_id).await {
        Ok(complete_user) => {
            let response = UserResponse {
                id: complete_user.church_user_id,
                auth_user_id: complete_user.auth_user_id.clone(),
                email: complete_user.email().to_string(),
                full_name: complete_user.full_name(),
                role: complete_user.role().to_string(),
                is_email_verified: complete_user.is_email_verified(),
                created_at: complete_user.created_at.to_string(),
                updated_at: complete_user.updated_at.to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// GET /v1/users
/// List all church users (paginated)
pub async fn list_users(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListUsersQuery>,
    _user: UserContext, // Requires authentication
) -> Result<HttpResponse> {
    use models::User;
    use sea_orm::{EntityTrait, PaginatorTrait};

    let page = if query.page < 1 { 1 } else { query.page };
    let limit = if query.limit < 1 || query.limit > 100 {
        20
    } else {
        query.limit
    };

    match User::find()
        .paginate(&**db, limit as u64)
        .fetch_page((page - 1) as u64)
        .await
    {
        Ok(users) => {
            let total = User::find().count(&**db).await.unwrap_or(0);

            Ok(HttpResponse::Ok().json(json!({
                "data": users,
                "pagination": {
                    "page": page,
                    "limit": limit,
                    "total": total,
                    "total_pages": (total as f64 / limit as f64).ceil() as i64
                }
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Configure user routes
pub fn configure_users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/link", web::post().to(link_user))
            .route("/{id}", web::get().to(get_user))
            .route("", web::get().to(list_users)),
    );
}
