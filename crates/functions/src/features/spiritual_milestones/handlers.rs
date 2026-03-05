use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::{CreateSpiritualMilestoneRequest, UpdateSpiritualMilestoneRequest};

use super::service::SpiritualMilestoneService;
use crate::features::users::service::UserService;

pub async fn create_milestone(
    db: web::Data<sea_orm::DatabaseConnection>,
    body: web::Json<CreateSpiritualMilestoneRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;

    let milestone = SpiritualMilestoneService::create(&db, user.id, body.into_inner()).await?;

    let response = create_response(milestone, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(response))
}

pub async fn list_milestones(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;

    let milestones = SpiritualMilestoneService::list_by_user(&db, user.id).await?;

    let response = create_response(milestones, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_milestone(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let milestone_id = path.into_inner();

    let milestone = SpiritualMilestoneService::get_by_id(&db, user.id, milestone_id).await?;

    let response = create_response(milestone, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn update_milestone(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    body: web::Json<UpdateSpiritualMilestoneRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let milestone_id = path.into_inner();

    let milestone =
        SpiritualMilestoneService::update(&db, user.id, milestone_id, body.into_inner()).await?;

    let response = create_response(milestone, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_milestone(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let milestone_id = path.into_inner();

    SpiritualMilestoneService::delete(&db, user.id, milestone_id).await?;

    let response = create_response("Spiritual milestone deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}
