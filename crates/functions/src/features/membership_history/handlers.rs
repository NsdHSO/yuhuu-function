use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::{CreateMembershipHistoryRequest, UpdateMembershipHistoryRequest};

use crate::features::users::service::UserService;
use super::service::MembershipHistoryService;

pub async fn create_membership(
    db: web::Data<sea_orm::DatabaseConnection>,
    body: web::Json<CreateMembershipHistoryRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;

    let membership = MembershipHistoryService::create(&db, user.id, body.into_inner()).await?;

    let response = create_response(membership, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(response))
}

pub async fn list_memberships(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;

    let memberships = MembershipHistoryService::list_by_user(&db, user.id).await?;

    let response = create_response(memberships, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_membership(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let membership_id = path.into_inner();

    let membership = MembershipHistoryService::get_by_id(&db, user.id, membership_id).await?;

    let response = create_response(membership, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn update_membership(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    body: web::Json<UpdateMembershipHistoryRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let membership_id = path.into_inner();

    let membership =
        MembershipHistoryService::update(&db, user.id, membership_id, body.into_inner()).await?;

    let response = create_response(membership, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_membership(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let membership_id = path.into_inner();

    MembershipHistoryService::delete(&db, user.id, membership_id).await?;

    let response = create_response("Membership history deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}
