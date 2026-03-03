use serde::{Deserialize, Serialize};
use crate::dto::SpiritualMilestoneModel;

#[derive(Debug, Deserialize)]
pub struct CreateSpiritualMilestoneRequest {
    pub milestone_type: String,
    pub milestone_date: Option<String>,
    pub location: Option<String>,
    pub officiant: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpiritualMilestoneRequest {
    pub milestone_type: Option<String>,
    pub milestone_date: Option<String>,
    pub location: Option<String>,
    pub officiant: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SpiritualMilestoneResponse {
    pub id: i64,
    pub user_id: i64,
    pub milestone_type: String,
    pub milestone_date: Option<String>,
    pub location: Option<String>,
    pub officiant: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<SpiritualMilestoneModel> for SpiritualMilestoneResponse {
    fn from(model: SpiritualMilestoneModel) -> Self {
        SpiritualMilestoneResponse {
            id: model.id,
            user_id: model.user_id,
            milestone_type: model.milestone_type,
            milestone_date: model.milestone_date.map(|d| d.to_string()),
            location: model.location,
            officiant: model.officiant,
            notes: model.notes,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
