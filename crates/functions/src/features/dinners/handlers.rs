use actix_web::{web, HttpResponse, Result};
use models::internal::{
    AddParticipantRequest, CreateDinnerRequest,
};
use http_response::{create_response, HttpCodeW};

use super::service::DinnerService;

pub async fn create_dinner(
    db: web::Data<sea_orm::DatabaseConnection>,
    req: web::Json<CreateDinnerRequest>,
) -> Result<HttpResponse> {
    let dinner = DinnerService::create_dinner(&db, req.into_inner(), None).await?;
    let resp = create_response(dinner, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn get_dinner(
    db: web::Data<sea_orm::DatabaseConnection>,
    dinner_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let dinner = DinnerService::get_dinner(&db, dinner_id.into_inner()).await?;
    let resp = create_response(dinner, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn list_dinners(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListDinnersQuery>,
) -> Result<HttpResponse> {
    let result = DinnerService::list_dinners(&db, query.page, query.limit).await?;
    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn add_participant(
    db: web::Data<sea_orm::DatabaseConnection>,
    dinner_id: web::Path<i64>,
    req: web::Json<AddParticipantRequest>,
) -> Result<HttpResponse> {
    let participant = DinnerService::add_participant(&db, dinner_id.into_inner(), req.into_inner(), None).await?;
    let resp = create_response(participant, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn get_dinner_participants(
    db: web::Data<sea_orm::DatabaseConnection>,
    dinner_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let result = DinnerService::get_dinner_with_participants(&db, dinner_id.into_inner()).await?;
    let resp = create_response(result, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn remove_participant(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse> {
    let (dinner_id, participant_id) = path.into_inner();
    DinnerService::remove_participant(&db, dinner_id, participant_id).await?;
    let resp = create_response(serde_json::json!({ "message": "Participant removed" }), HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

#[derive(serde::Deserialize)]
pub struct ListDinnersQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}
