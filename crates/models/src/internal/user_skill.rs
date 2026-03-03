use serde::{Deserialize, Serialize};
use crate::dto::UserSkillModel;

#[derive(Debug, Deserialize)]
pub struct CreateUserSkillRequest {
    pub skill_name: String,
    pub skill_category: Option<String>,
    pub proficiency_level: Option<String>,
    pub years_of_experience: Option<i32>,
    pub is_willing_to_serve: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserSkillRequest {
    pub skill_name: Option<String>,
    pub skill_category: Option<String>,
    pub proficiency_level: Option<String>,
    pub years_of_experience: Option<i32>,
    pub is_willing_to_serve: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserSkillResponse {
    pub id: i64,
    pub user_id: i64,
    pub skill_name: String,
    pub skill_category: Option<String>,
    pub proficiency_level: Option<String>,
    pub years_of_experience: Option<i32>,
    pub is_willing_to_serve: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<UserSkillModel> for UserSkillResponse {
    fn from(model: UserSkillModel) -> Self {
        UserSkillResponse {
            id: model.id,
            user_id: model.user_id,
            skill_name: model.skill_name,
            skill_category: model.skill_category,
            proficiency_level: model.proficiency_level,
            years_of_experience: model.years_of_experience,
            is_willing_to_serve: model.is_willing_to_serve,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
