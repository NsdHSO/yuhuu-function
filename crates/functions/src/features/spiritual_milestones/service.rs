use http_response::{CustomError, HttpCodeW};
use models::dto::{SpiritualMilestone, SpiritualMilestoneActiveModel};
use models::internal::{
    CreateSpiritualMilestoneRequest, SpiritualMilestoneResponse, UpdateSpiritualMilestoneRequest,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

pub struct SpiritualMilestoneService;

impl SpiritualMilestoneService {
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i64,
        request: CreateSpiritualMilestoneRequest,
    ) -> Result<SpiritualMilestoneResponse, CustomError> {
        let milestone_date = if let Some(date_str) = request.milestone_date {
            Some(
                chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
                    CustomError::new(
                        HttpCodeW::BadRequest,
                        "Invalid date format. Use YYYY-MM-DD".to_string(),
                    )
                })?,
            )
        } else {
            None
        };

        let now = chrono::Utc::now().naive_utc();

        let new_milestone = SpiritualMilestoneActiveModel {
            user_id: Set(user_id),
            milestone_type: Set(request.milestone_type),
            milestone_date: Set(milestone_date),
            location: Set(request.location),
            officiant: Set(request.officiant),
            notes: Set(request.notes),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let milestone = new_milestone.insert(db).await.map_err(|e| {
            if e.to_string().contains("duplicate key") {
                CustomError::new(
                    HttpCodeW::Conflict,
                    "A milestone of this type already exists for this user".to_string(),
                )
            } else {
                CustomError::from(e)
            }
        })?;

        Ok(milestone.into())
    }

    pub async fn list_by_user(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<SpiritualMilestoneResponse>, CustomError> {
        use models::dto::spiritual_milestone::Column;

        let milestones = SpiritualMilestone::find()
            .filter(Column::UserId.eq(user_id))
            .all(db)
            .await?;

        Ok(milestones.into_iter().map(|m| m.into()).collect())
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        user_id: i64,
        milestone_id: i64,
    ) -> Result<SpiritualMilestoneResponse, CustomError> {
        use models::dto::spiritual_milestone::Column;

        let milestone = SpiritualMilestone::find_by_id(milestone_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Spiritual milestone not found".to_string())
            })?;

        Ok(milestone.into())
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_id: i64,
        milestone_id: i64,
        request: UpdateSpiritualMilestoneRequest,
    ) -> Result<SpiritualMilestoneResponse, CustomError> {
        use models::dto::spiritual_milestone::Column;

        let existing = SpiritualMilestone::find_by_id(milestone_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Spiritual milestone not found".to_string())
            })?;

        let mut active: SpiritualMilestoneActiveModel = existing.into();

        if let Some(milestone_type) = request.milestone_type {
            active.milestone_type = Set(milestone_type);
        }
        if let Some(date_str) = request.milestone_date {
            let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    "Invalid date format. Use YYYY-MM-DD".to_string(),
                )
            })?;
            active.milestone_date = Set(Some(date));
        }
        if request.location.is_some() {
            active.location = Set(request.location);
        }
        if request.officiant.is_some() {
            active.officiant = Set(request.officiant);
        }
        if request.notes.is_some() {
            active.notes = Set(request.notes);
        }

        active.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated = active.update(db).await.map_err(|e| {
            if e.to_string().contains("duplicate key") {
                CustomError::new(
                    HttpCodeW::Conflict,
                    "A milestone of this type already exists for this user".to_string(),
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
        milestone_id: i64,
    ) -> Result<(), CustomError> {
        use models::dto::spiritual_milestone::Column;

        let milestone = SpiritualMilestone::find_by_id(milestone_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Spiritual milestone not found".to_string())
            })?;

        let active: SpiritualMilestoneActiveModel = milestone.into();
        active.delete(db).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_milestone_request() {
        let request = CreateSpiritualMilestoneRequest {
            milestone_type: "baptism".to_string(),
            milestone_date: Some("2020-03-15".to_string()),
            location: Some("Grace Community Church".to_string()),
            officiant: Some("Pastor John Smith".to_string()),
            notes: Some("Easter baptism service".to_string()),
        };

        assert_eq!(request.milestone_type, "baptism");
        assert_eq!(request.milestone_date.clone().unwrap(), "2020-03-15");
        assert_eq!(request.location.clone().unwrap(), "Grace Community Church");
    }

    #[test]
    fn test_create_milestone_minimal() {
        let request = CreateSpiritualMilestoneRequest {
            milestone_type: "conversion".to_string(),
            milestone_date: None,
            location: None,
            officiant: None,
            notes: None,
        };

        assert_eq!(request.milestone_type, "conversion");
        assert!(request.milestone_date.is_none());
        assert!(request.location.is_none());
    }

    #[test]
    fn test_update_milestone_request() {
        let request = UpdateSpiritualMilestoneRequest {
            milestone_type: Some("water_baptism".to_string()),
            milestone_date: Some("2021-06-20".to_string()),
            location: None,
            officiant: Some("Pastor Jane Doe".to_string()),
            notes: None,
        };

        assert_eq!(request.milestone_type.clone().unwrap(), "water_baptism");
        assert_eq!(request.milestone_date.clone().unwrap(), "2021-06-20");
        assert!(request.location.is_none());
    }

    #[test]
    fn test_response_fields() {
        let now = chrono::Utc::now().naive_utc();

        let response = SpiritualMilestoneResponse {
            id: 1,
            user_id: 10,
            milestone_type: "baptism".to_string(),
            milestone_date: Some("2020-03-15".to_string()),
            location: Some("Grace Community Church".to_string()),
            officiant: Some("Pastor John".to_string()),
            notes: Some("Special service".to_string()),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(response.id, 1);
        assert_eq!(response.milestone_type, "baptism");
        assert_eq!(response.location.clone().unwrap(), "Grace Community Church");
    }

    #[test]
    fn test_milestone_types() {
        let types = vec![
            "conversion",
            "baptism",
            "water_baptism",
            "spirit_baptism",
            "confirmation",
            "dedication",
            "ordination",
        ];

        for milestone_type in types {
            let request = CreateSpiritualMilestoneRequest {
                milestone_type: milestone_type.to_string(),
                milestone_date: None,
                location: None,
                officiant: None,
                notes: None,
            };
            assert_eq!(request.milestone_type, milestone_type);
        }
    }
}
