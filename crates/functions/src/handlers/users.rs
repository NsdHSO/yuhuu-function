use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use models::internal::ListUsersQuery;
use models::{User, UserActiveModel};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct LinkUserRequest {
    // No fields needed - we get the sub from JWT token
}

#[derive(Debug, Serialize)]
pub struct LinkUserResponse {
    pub id: i64,
    pub auth_user_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub message: String,
}

/// POST /v1/users/link
/// Link the authenticated user from auth server to church system
pub async fn link_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject, // Get authenticated user's sub from JWT
) -> Result<HttpResponse> {
    // Check if user already exists
    use models::dto::user::Column;
    let existing = User::find()
        .filter(Column::AuthUserId.eq(&subject.sub))
        .one(&**db)
        .await;

    match existing {
        Ok(Some(user)) => {
            // User already linked
            Ok(HttpResponse::Ok().json(LinkUserResponse {
                id: user.id,
                auth_user_id: user.auth_user_id,
                created_at: user.created_at,
                updated_at: user.updated_at,
                message: "User already linked".to_string(),
            }))
        }
        Ok(None) => {
            // Create new church user linked to auth user
            let now = chrono::Utc::now().naive_utc();
            let new_user = UserActiveModel {
                auth_user_id: Set(subject.sub.clone()),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };

            match new_user.insert(&**db).await {
                Ok(user) => Ok(HttpResponse::Created().json(LinkUserResponse {
                    id: user.id,
                    auth_user_id: user.auth_user_id,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    message: "User linked successfully".to_string(),
                })),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to create user: {}", e)
                }))),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Database error: {}", e)
        }))),
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub auth_user_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// GET /v1/users/:id
/// Get a specific church user by ID
pub async fn get_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject, // Requires authentication
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match User::find_by_id(user_id).one(&**db).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(UserResponse {
            id: user.id,
            auth_user_id: user.auth_user_id,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "User not found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Database error: {}", e)
        }))),
    }
}

/// GET /v1/users
/// List all church users (paginated)
pub async fn list_users(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListUsersQuery>,
    _subject: Subject, // Requires authentication
) -> Result<HttpResponse> {
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
            .route("", web::get().to(list_users))
            .route("/link", web::post().to(link_user))
            .route("/{id}", web::get().to(get_user)),
    );
}
