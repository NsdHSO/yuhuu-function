use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, CustomError, HttpCodeW};
use models::internal::{BootstrapCreated, BootstrapRequest, BootstrapResponse};

use crate::features::profiles::service::ProfileService;
use crate::features::users::service::UserService;

/// POST /v1/bootstrap
/// - Ensures a church user exists for the JWT subject (links if needed)
/// - Fetches profile; optionally creates it when missing if request asks so
pub async fn bootstrap(
    db: web::Data<sea_orm::DatabaseConnection>,
    body: Option<web::Json<BootstrapRequest>>,
    subject: Subject,
) -> Result<HttpResponse> {
    let mut created = BootstrapCreated::default();

    // 1) Link (idempotent)
    let link_res = UserService::link_user(&db, &subject.sub).await?;
    created.linked = !link_res.message.contains("already");

    // 2) Load user for id
    let user = UserService::get_user_by_id(&db, link_res.id).await?;

    // 3) Profile: fetch, or create if asked and missing
    let profile = match ProfileService::get_profile(&db, user.id).await {
        Ok(p) => Some(p),
        Err(err) if matches!(err.error_status_code, HttpCodeW::NotFound) => {
            // missing
            if let Some(req) = &body {
                if req.create_profile_if_missing {
if let Some(payload) = req.profile.as_ref() {
                        let created_profile =
ProfileService::create_profile(&db, user.id, payload.clone()).await?;
                        created.profile = true;
                        Some(created_profile)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        Err(e) => return Err(e.into()),
    };

    let response = BootstrapResponse {
        user,
        profile,
        created,
    };
    let response = create_response(response, HttpCodeW::Created);

    Ok(HttpResponse::Ok().json(response))
}