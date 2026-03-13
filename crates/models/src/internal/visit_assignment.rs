use crate::dto::VisitAssignmentModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AssignmentListQuery {
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[serde(default)]
    pub offset: u64,
}

#[derive(Debug, Deserialize)]
pub struct MyAssignmentsQuery {
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[serde(default)]
    pub offset: u64,
    pub status: Option<String>,
}

fn default_limit() -> u64 {
    50
}

#[derive(Debug, Deserialize)]
pub struct CreateVisitAssignmentRequest {
    pub family_id: i64,
    pub assigned_to_user_id: i64,
    pub scheduled_date: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVisitAssignmentRequest {
    pub scheduled_date: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MarkArrivalRequest {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct MarkCompleteRequest {
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VisitableFamilyBrief {
    pub id: i64,
    pub family_name: String,
    pub address_street: String,
    pub address_city: String,
}

#[derive(Debug, Serialize)]
pub struct AssignedUserBrief {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct VisitAssignmentResponse {
    pub id: i64,
    pub family_id: i64,
    pub family: Option<VisitableFamilyBrief>,
    pub assigned_to_user_id: i64,
    pub assigned_user: Option<AssignedUserBrief>,
    pub scheduled_date: String,
    pub status: String,
    pub arrived_at: Option<chrono::NaiveDateTime>,
    pub arrived_latitude: Option<f64>,
    pub arrived_longitude: Option<f64>,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<VisitAssignmentModel> for VisitAssignmentResponse {
    fn from(model: VisitAssignmentModel) -> Self {
        VisitAssignmentResponse {
            id: model.id,
            family_id: model.family_id,
            family: None,
            assigned_to_user_id: model.assigned_to_user_id,
            assigned_user: None,
            scheduled_date: model.scheduled_date.to_string(),
            status: model.status,
            arrived_at: model.arrived_at,
            arrived_latitude: model.arrived_latitude.and_then(|d| d.to_string().parse().ok()),
            arrived_longitude: model.arrived_longitude.and_then(|d| d.to_string().parse().ok()),
            completed_at: model.completed_at,
            notes: model.notes,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
