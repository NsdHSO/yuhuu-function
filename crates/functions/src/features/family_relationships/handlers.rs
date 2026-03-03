use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::{CreateFamilyRelationshipRequest, UpdateFamilyRelationshipRequest};

use crate::features::users::service::UserService;
use super::service::FamilyRelationshipService;

pub async fn create_family_relationship(
    db: web::Data<sea_orm::DatabaseConnection>,
    body: web::Json<CreateFamilyRelationshipRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;

    let relationship =
        FamilyRelationshipService::create(&db, user.id, body.into_inner()).await?;

    let response = create_response(relationship, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(response))
}

pub async fn list_family_relationships(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;

    let relationships = FamilyRelationshipService::list_by_user(&db, user.id).await?;

    let response = create_response(relationships, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_family_relationship(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let relationship_id = path.into_inner();

    let relationship = FamilyRelationshipService::get_by_id(&db, user.id, relationship_id).await?;

    let response = create_response(relationship, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn update_family_relationship(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    body: web::Json<UpdateFamilyRelationshipRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let relationship_id = path.into_inner();

    let relationship =
        FamilyRelationshipService::update(&db, user.id, relationship_id, body.into_inner()).await?;

    let response = create_response(relationship, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_family_relationship(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let relationship_id = path.into_inner();

    FamilyRelationshipService::delete(&db, user.id, relationship_id).await?;

    let response = create_response("Family relationship deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(response))
}
