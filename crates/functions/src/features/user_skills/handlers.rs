use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::{CreateUserSkillRequest, UpdateUserSkillRequest};

use super::service::UserSkillService;
use crate::features::users::service::UserService;

pub async fn create_skill(
    db: web::Data<sea_orm::DatabaseConnection>,
    body: web::Json<CreateUserSkillRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;

    let skill = UserSkillService::create(&db, user.id, body.into_inner()).await?;

    let response = create_response(skill, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(response))
}

pub async fn list_skills(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;

    let skills = UserSkillService::list_by_user(&db, user.id).await?;

    let response = create_response(skills, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_skill(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let skill_id = path.into_inner();

    let skill = UserSkillService::get_by_id(&db, user.id, skill_id).await?;

    let response = create_response(skill, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn update_skill(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    body: web::Json<UpdateUserSkillRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let skill_id = path.into_inner();

    let skill = UserSkillService::update(&db, user.id, skill_id, body.into_inner()).await?;

    let response = create_response(skill, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_skill(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let skill_id = path.into_inner();

    UserSkillService::delete(&db, user.id, skill_id).await?;

    let response = create_response("Skill deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}
