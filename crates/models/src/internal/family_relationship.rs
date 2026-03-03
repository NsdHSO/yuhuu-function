use serde::{Deserialize, Serialize};
use crate::dto::FamilyRelationshipModel;

#[derive(Debug, Deserialize)]
pub struct CreateFamilyRelationshipRequest {
    pub related_user_id: Option<i64>,
    pub related_person_name: Option<String>,
    pub related_person_dob: Option<String>,
    pub related_person_phone: Option<String>,
    pub related_person_email: Option<String>,
    pub relationship_type: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFamilyRelationshipRequest {
    pub related_user_id: Option<i64>,
    pub related_person_name: Option<String>,
    pub related_person_dob: Option<String>,
    pub related_person_phone: Option<String>,
    pub related_person_email: Option<String>,
    pub relationship_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FamilyRelationshipResponse {
    pub id: i64,
    pub user_id: i64,
    pub related_user_id: Option<i64>,
    pub related_person_name: Option<String>,
    pub related_person_dob: Option<String>,
    pub related_person_phone: Option<String>,
    pub related_person_email: Option<String>,
    pub relationship_type: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<FamilyRelationshipModel> for FamilyRelationshipResponse {
    fn from(model: FamilyRelationshipModel) -> Self {
        FamilyRelationshipResponse {
            id: model.id,
            user_id: model.user_id,
            related_user_id: model.related_user_id,
            related_person_name: model.related_person_name,
            related_person_dob: model.related_person_dob.map(|d| d.to_string()),
            related_person_phone: model.related_person_phone,
            related_person_email: model.related_person_email,
            relationship_type: model.relationship_type,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
