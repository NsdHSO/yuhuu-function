use actix_web::{web, HttpResponse, Result};
use auth_integration::AdminGuard;
use http_response::{create_response, HttpCodeW};
use models::internal::{
    AssignmentListQuery, CreateVisitAssignmentRequest, CreateVisitableFamilyRequest,
    FamilyListQuery, UpdateVisitableFamilyRequest,
};

use crate::features::admin::service::AdminService;

// Family Management Handlers

pub async fn list_families(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<FamilyListQuery>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let families = AdminService::list_families(&db, query.into_inner()).await?;
    let resp = create_response(families, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_family(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i64>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let family = AdminService::get_family(&db, id.into_inner()).await?;
    let resp = create_response(family, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn create_family(
    db: web::Data<sea_orm::DatabaseConnection>,
    req: web::Json<CreateVisitableFamilyRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let family = AdminService::create_family(&db, req.into_inner()).await?;
    let resp = create_response(family, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn update_family(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i64>,
    req: web::Json<UpdateVisitableFamilyRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let family = AdminService::update_family(&db, id.into_inner(), req.into_inner()).await?;
    let resp = create_response(family, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_family(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i64>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    AdminService::delete_family(&db, id.into_inner()).await?;
    let resp = create_response("Family deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

// Assignment Management Handlers

pub async fn list_assignments(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<AssignmentListQuery>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let assignments = AdminService::list_assignments(&db, query.into_inner()).await?;
    let resp = create_response(assignments, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn create_assignment(
    db: web::Data<sea_orm::DatabaseConnection>,
    req: web::Json<CreateVisitAssignmentRequest>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    let assignment = AdminService::create_assignment(&db, req.into_inner()).await?;
    let resp = create_response(assignment, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn delete_assignment(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i64>,
    _admin: AdminGuard,
) -> Result<HttpResponse> {
    AdminService::delete_assignment(&db, id.into_inner()).await?;
    let resp = create_response("Assignment deleted successfully", HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
