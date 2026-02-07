use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use models::internal::{ListUsersQuery};
use serde_json::json;

// Removed link_user and get_user endpoints that depended on UserService
// If you need these, implement them with direct database access

/// GET /v1/users
/// List all church users (paginated)
pub async fn list_users(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListUsersQuery>,
    _user: Subject, // Requires authentication
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
            .route("", web::get().to(list_users)),
    );
}
