use crate::dto::MembershipHistoryModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMembershipHistoryRequest {
    pub church_name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub transfer_type: Option<String>,
    pub previous_role: Option<String>,
    pub transfer_letter_received: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMembershipHistoryRequest {
    pub church_name: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub transfer_type: Option<String>,
    pub previous_role: Option<String>,
    pub transfer_letter_received: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MembershipHistoryResponse {
    pub id: i64,
    pub user_id: i64,
    pub church_name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub transfer_type: Option<String>,
    pub previous_role: Option<String>,
    pub transfer_letter_received: Option<bool>,
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<MembershipHistoryModel> for MembershipHistoryResponse {
    fn from(model: MembershipHistoryModel) -> Self {
        MembershipHistoryResponse {
            id: model.id,
            user_id: model.user_id,
            church_name: model.church_name,
            start_date: model.start_date.map(|d| d.to_string()),
            end_date: model.end_date.map(|d| d.to_string()),
            transfer_type: model.transfer_type,
            previous_role: model.previous_role,
            transfer_letter_received: model.transfer_letter_received,
            notes: model.notes,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
