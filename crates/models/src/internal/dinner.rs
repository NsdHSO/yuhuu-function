use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateDinnerRequest {
    pub dinner_date: String,
    pub meal_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DinnerResponse {
    pub id: i64,
    pub uuid: uuid::Uuid,
    pub dinner_date: chrono::NaiveDate,
    pub meal_type: String,
    pub description: Option<String>,
    pub recorded_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct AddParticipantRequest {
    pub user_id: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ParticipantResponse {
    pub id: i64,
    pub uuid: uuid::Uuid,
    pub dinner_id: i64,
    pub user_id: i64,
    pub notes: Option<String>,
    pub recorded_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct DinnerWithParticipantsResponse {
    pub dinner: DinnerResponse,
    pub participants: Vec<ParticipantResponse>,
}
