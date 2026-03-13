use actix_web::{web, HttpResponse, Result};
use auth_integration::{AdminGuard, Subject};
use http_response::{create_response, CustomError, HttpCodeW};
use models::internal::{
    MarkArrivalRequest, MarkCompleteRequest, MyAssignmentsQuery, UpdateVisitAssignmentRequest,
};

use crate::features::users::service::UserService;
use crate::features::visits::services::VisitAssignmentService;

async fn get_church_user_id(
    db: &sea_orm::DatabaseConnection,
    auth_id: &str,
) -> Result<i64, CustomError> {
    let user = UserService::get_user_by_auth_id(db, auth_id).await?;
    Ok(user.id)
}

async fn verify_ownership(
    db: &sea_orm::DatabaseConnection,
    assignment_id: i64,
    user_id: i64,
    is_admin: bool,
) -> Result<(), CustomError> {
    if is_admin {
        return Ok(());
    }
    let assignment = VisitAssignmentService::get_by_id(db, assignment_id).await?;
    if assignment.assigned_to_user_id != user_id {
        return Err(CustomError::new(HttpCodeW::Forbidden, "Access denied".to_string()));
    }
    Ok(())
}

pub async fn list_my_assignments(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<MyAssignmentsQuery>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user_id = get_church_user_id(&db, &subject.sub).await?;
    let assignments =
        VisitAssignmentService::list_by_user(&db, user_id, query.status.clone(), query.limit, query.offset).await?;
    let resp = create_response(assignments, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_assignment(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i64>,
    subject: Subject,
    admin: Option<AdminGuard>,
) -> Result<HttpResponse> {
    let user_id = get_church_user_id(&db, &subject.sub).await?;
    let assignment_id = id.into_inner();
    verify_ownership(&db, assignment_id, user_id, admin.is_some()).await?;
    let assignment = VisitAssignmentService::get_by_id(&db, assignment_id).await?;
    let resp = create_response(assignment, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn update_assignment(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i64>,
    req: web::Json<UpdateVisitAssignmentRequest>,
    subject: Subject,
    admin: Option<AdminGuard>,
) -> Result<HttpResponse> {
    let user_id = get_church_user_id(&db, &subject.sub).await?;
    let assignment_id = id.into_inner();
    verify_ownership(&db, assignment_id, user_id, admin.is_some()).await?;

    let assignment = if admin.is_some() {
        VisitAssignmentService::update_admin(&db, assignment_id, req.into_inner()).await?
    } else {
        VisitAssignmentService::update_user_notes(&db, assignment_id, req.notes.clone()).await?
    };

    let resp = create_response(assignment, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn mark_arrival(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i64>,
    req: web::Json<MarkArrivalRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user_id = get_church_user_id(&db, &subject.sub).await?;
    let assignment_id = id.into_inner();
    verify_ownership(&db, assignment_id, user_id, false).await?;
    let assignment = VisitAssignmentService::mark_arrival(&db, assignment_id, req.into_inner()).await?;
    let resp = create_response(assignment, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn mark_complete(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i64>,
    req: web::Json<MarkCompleteRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user_id = get_church_user_id(&db, &subject.sub).await?;
    let assignment_id = id.into_inner();
    verify_ownership(&db, assignment_id, user_id, false).await?;
    let assignment = VisitAssignmentService::mark_complete(&db, assignment_id, req.into_inner()).await?;
    let resp = create_response(assignment, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
