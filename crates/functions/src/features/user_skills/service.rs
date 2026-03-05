use http_response::{CustomError, HttpCodeW};
use models::dto::{UserSkill, UserSkillActiveModel};
use models::internal::{CreateUserSkillRequest, UpdateUserSkillRequest, UserSkillResponse};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct UserSkillService;

impl UserSkillService {
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i64,
        request: CreateUserSkillRequest,
    ) -> Result<UserSkillResponse, CustomError> {
        if let Some(years) = request.years_of_experience {
            if years < 0 {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    "Years of experience cannot be negative".to_string(),
                ));
            }
        }

        let now = chrono::Utc::now().naive_utc();

        let new_skill = UserSkillActiveModel {
            user_id: Set(user_id),
            skill_name: Set(request.skill_name),
            skill_category: Set(request.skill_category),
            proficiency_level: Set(request.proficiency_level),
            years_of_experience: Set(request.years_of_experience),
            is_willing_to_serve: Set(request.is_willing_to_serve.unwrap_or(true)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let skill = new_skill.insert(db).await.map_err(|e| {
            if e.to_string().contains("duplicate key")
                || e.to_string().contains("idx_user_skills_user_skill_unique")
            {
                CustomError::new(
                    HttpCodeW::Conflict,
                    "You have already added this skill. Please update the existing skill instead."
                        .to_string(),
                )
            } else {
                CustomError::from(e)
            }
        })?;

        Ok(skill.into())
    }

    pub async fn list_by_user(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<UserSkillResponse>, CustomError> {
        use models::dto::user_skill::Column;

        let skills = UserSkill::find()
            .filter(Column::UserId.eq(user_id))
            .all(db)
            .await?;

        Ok(skills.into_iter().map(|s| s.into()).collect())
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        user_id: i64,
        skill_id: i64,
    ) -> Result<UserSkillResponse, CustomError> {
        use models::dto::user_skill::Column;

        let skill = UserSkill::find_by_id(skill_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Skill not found".to_string()))?;

        Ok(skill.into())
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_id: i64,
        skill_id: i64,
        request: UpdateUserSkillRequest,
    ) -> Result<UserSkillResponse, CustomError> {
        use models::dto::user_skill::Column;

        let existing = UserSkill::find_by_id(skill_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Skill not found".to_string()))?;

        let mut active: UserSkillActiveModel = existing.into();

        if let Some(skill_name) = request.skill_name {
            active.skill_name = Set(skill_name);
        }
        if request.skill_category.is_some() {
            active.skill_category = Set(request.skill_category);
        }
        if request.proficiency_level.is_some() {
            active.proficiency_level = Set(request.proficiency_level);
        }
        if let Some(years) = request.years_of_experience {
            if years < 0 {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    "Years of experience cannot be negative".to_string(),
                ));
            }
            active.years_of_experience = Set(Some(years));
        }
        if let Some(is_willing) = request.is_willing_to_serve {
            active.is_willing_to_serve = Set(is_willing);
        }

        active.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated = active.update(db).await.map_err(|e| {
            if e.to_string().contains("duplicate key")
                || e.to_string().contains("idx_user_skills_user_skill_unique")
            {
                CustomError::new(
                    HttpCodeW::Conflict,
                    "You have already added this skill. Please update the existing skill instead."
                        .to_string(),
                )
            } else {
                CustomError::from(e)
            }
        })?;

        Ok(updated.into())
    }

    pub async fn delete(
        db: &DatabaseConnection,
        user_id: i64,
        skill_id: i64,
    ) -> Result<(), CustomError> {
        use models::dto::user_skill::Column;

        let skill = UserSkill::find_by_id(skill_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Skill not found".to_string()))?;

        let active: UserSkillActiveModel = skill.into();
        active.delete(db).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_skill_request_full() {
        let request = CreateUserSkillRequest {
            skill_name: "Guitar Playing".to_string(),
            skill_category: Some("Music".to_string()),
            proficiency_level: Some("advanced".to_string()),
            years_of_experience: Some(10),
            is_willing_to_serve: Some(true),
        };

        assert_eq!(request.skill_name, "Guitar Playing");
        assert_eq!(request.skill_category.clone().unwrap(), "Music");
        assert_eq!(request.proficiency_level.clone().unwrap(), "advanced");
        assert_eq!(request.years_of_experience.unwrap(), 10);
        assert_eq!(request.is_willing_to_serve.unwrap(), true);
    }

    #[test]
    fn test_create_skill_request_minimal() {
        let request = CreateUserSkillRequest {
            skill_name: "Public Speaking".to_string(),
            skill_category: None,
            proficiency_level: None,
            years_of_experience: None,
            is_willing_to_serve: None,
        };

        assert_eq!(request.skill_name, "Public Speaking");
        assert!(request.skill_category.is_none());
        assert!(request.proficiency_level.is_none());
        assert!(request.years_of_experience.is_none());
    }

    #[test]
    fn test_update_skill_request() {
        let request = UpdateUserSkillRequest {
            skill_name: None,
            skill_category: Some("Teaching".to_string()),
            proficiency_level: Some("expert".to_string()),
            years_of_experience: Some(15),
            is_willing_to_serve: Some(false),
        };

        assert!(request.skill_name.is_none());
        assert_eq!(request.skill_category.clone().unwrap(), "Teaching");
        assert_eq!(request.proficiency_level.clone().unwrap(), "expert");
        assert_eq!(request.years_of_experience.unwrap(), 15);
        assert_eq!(request.is_willing_to_serve.unwrap(), false);
    }

    #[test]
    fn test_response_fields() {
        let now = chrono::Utc::now().naive_utc();

        let response = UserSkillResponse {
            id: 1,
            user_id: 10,
            skill_name: "Guitar Playing".to_string(),
            skill_category: Some("Music".to_string()),
            proficiency_level: Some("advanced".to_string()),
            years_of_experience: Some(10),
            is_willing_to_serve: true,
            created_at: now,
            updated_at: now,
        };

        assert_eq!(response.id, 1);
        assert_eq!(response.skill_name, "Guitar Playing");
        assert_eq!(response.skill_category.clone().unwrap(), "Music");
        assert_eq!(response.is_willing_to_serve, true);
    }

    #[test]
    fn test_proficiency_levels() {
        let levels = vec!["beginner", "intermediate", "advanced", "expert"];

        for level in levels {
            let request = CreateUserSkillRequest {
                skill_name: "Test Skill".to_string(),
                skill_category: None,
                proficiency_level: Some(level.to_string()),
                years_of_experience: None,
                is_willing_to_serve: None,
            };
            assert_eq!(request.proficiency_level.clone().unwrap(), level);
        }
    }

    #[test]
    fn test_skill_categories() {
        let categories = vec![
            "Music",
            "Technology",
            "Teaching",
            "Administration",
            "Hospitality",
            "Creative Arts",
        ];

        for category in categories {
            let request = CreateUserSkillRequest {
                skill_name: "Test Skill".to_string(),
                skill_category: Some(category.to_string()),
                proficiency_level: None,
                years_of_experience: None,
                is_willing_to_serve: None,
            };
            assert_eq!(request.skill_category.clone().unwrap(), category);
        }
    }

    #[test]
    fn test_willing_to_serve_default() {
        let request = CreateUserSkillRequest {
            skill_name: "Test Skill".to_string(),
            skill_category: None,
            proficiency_level: None,
            years_of_experience: None,
            is_willing_to_serve: None,
        };

        assert!(request.is_willing_to_serve.is_none());
    }

    #[test]
    fn test_years_experience_validation() {
        let valid_years = vec![0, 1, 5, 10, 20, 50];

        for years in valid_years {
            let request = CreateUserSkillRequest {
                skill_name: "Test Skill".to_string(),
                skill_category: None,
                proficiency_level: None,
                years_of_experience: Some(years),
                is_willing_to_serve: None,
            };
            assert_eq!(request.years_of_experience.unwrap(), years);
        }
    }
}
