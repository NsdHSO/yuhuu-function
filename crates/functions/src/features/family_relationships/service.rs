use http_response::{CustomError, HttpCodeW};
use models::dto::{FamilyRelationship, FamilyRelationshipActiveModel};
use models::internal::{
    CreateFamilyRelationshipRequest, FamilyRelationshipResponse, UpdateFamilyRelationshipRequest,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct FamilyRelationshipService;

impl FamilyRelationshipService {
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i64,
        request: CreateFamilyRelationshipRequest,
    ) -> Result<FamilyRelationshipResponse, CustomError> {
        if request.related_user_id.is_none() && request.related_person_name.is_none() {
            return Err(CustomError::new(
                HttpCodeW::BadRequest,
                "Either related_user_id or related_person_name must be provided".to_string(),
            ));
        }

        let related_person_dob = if let Some(dob_str) = request.related_person_dob {
            Some(
                chrono::NaiveDate::parse_from_str(&dob_str, "%Y-%m-%d").map_err(|_| {
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

        let new_relationship = FamilyRelationshipActiveModel {
            user_id: Set(user_id),
            related_user_id: Set(request.related_user_id),
            related_person_name: Set(request.related_person_name),
            related_person_dob: Set(related_person_dob),
            related_person_phone: Set(request.related_person_phone),
            related_person_email: Set(request.related_person_email),
            relationship_type: Set(request.relationship_type),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let relationship = new_relationship.insert(db).await?;

        Ok(relationship.into())
    }

    pub async fn list_by_user(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<FamilyRelationshipResponse>, CustomError> {
        use models::dto::family_relationship::Column;

        let relationships = FamilyRelationship::find()
            .filter(Column::UserId.eq(user_id))
            .all(db)
            .await?;

        Ok(relationships.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        user_id: i64,
        relationship_id: i64,
    ) -> Result<FamilyRelationshipResponse, CustomError> {
        use models::dto::family_relationship::Column;

        let relationship = FamilyRelationship::find_by_id(relationship_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "Family relationship not found".to_string(),
                )
            })?;

        Ok(relationship.into())
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_id: i64,
        relationship_id: i64,
        request: UpdateFamilyRelationshipRequest,
    ) -> Result<FamilyRelationshipResponse, CustomError> {
        use models::dto::family_relationship::Column;

        let existing = FamilyRelationship::find_by_id(relationship_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "Family relationship not found".to_string(),
                )
            })?;

        let mut active: FamilyRelationshipActiveModel = existing.into();

        if let Some(related_user_id) = request.related_user_id {
            active.related_user_id = Set(Some(related_user_id));
        }
        if request.related_person_name.is_some() {
            active.related_person_name = Set(request.related_person_name);
        }
        if let Some(dob_str) = request.related_person_dob {
            let dob = chrono::NaiveDate::parse_from_str(&dob_str, "%Y-%m-%d").map_err(|_| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    "Invalid date format. Use YYYY-MM-DD".to_string(),
                )
            })?;
            active.related_person_dob = Set(Some(dob));
        }
        if request.related_person_phone.is_some() {
            active.related_person_phone = Set(request.related_person_phone);
        }
        if request.related_person_email.is_some() {
            active.related_person_email = Set(request.related_person_email);
        }
        if let Some(relationship_type) = request.relationship_type {
            active.relationship_type = Set(relationship_type);
        }

        active.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated = active.update(db).await?;

        Ok(updated.into())
    }

    pub async fn delete(
        db: &DatabaseConnection,
        user_id: i64,
        relationship_id: i64,
    ) -> Result<(), CustomError> {
        use models::dto::family_relationship::Column;

        let relationship = FamilyRelationship::find_by_id(relationship_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "Family relationship not found".to_string(),
                )
            })?;

        let active: FamilyRelationshipActiveModel = relationship.into();
        active.delete(db).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_request_with_registered_user() {
        let request = CreateFamilyRelationshipRequest {
            related_user_id: Some(5),
            related_person_name: None,
            related_person_dob: None,
            related_person_phone: None,
            related_person_email: None,
            relationship_type: "spouse".to_string(),
        };

        assert_eq!(request.related_user_id, Some(5));
        assert!(request.related_person_name.is_none());
        assert_eq!(request.relationship_type, "spouse");
    }

    #[test]
    fn test_create_request_with_unregistered_person() {
        let request = CreateFamilyRelationshipRequest {
            related_user_id: None,
            related_person_name: Some("Jane Doe".to_string()),
            related_person_dob: Some("1990-05-15".to_string()),
            related_person_phone: Some("+1234567890".to_string()),
            related_person_email: Some("jane@example.com".to_string()),
            relationship_type: "child".to_string(),
        };

        assert!(request.related_user_id.is_none());
        assert_eq!(request.related_person_name.clone().unwrap(), "Jane Doe");
        assert_eq!(request.related_person_dob.clone().unwrap(), "1990-05-15");
        assert_eq!(request.relationship_type, "child");
    }

    #[test]
    fn test_update_request_partial_fields() {
        let request = UpdateFamilyRelationshipRequest {
            related_user_id: None,
            related_person_name: Some("Updated Name".to_string()),
            related_person_dob: None,
            related_person_phone: Some("+9876543210".to_string()),
            related_person_email: None,
            relationship_type: Some("sibling".to_string()),
        };

        assert!(request.related_user_id.is_none());
        assert_eq!(request.related_person_name.clone().unwrap(), "Updated Name");
        assert!(request.related_person_dob.is_none());
        assert_eq!(request.relationship_type.clone().unwrap(), "sibling");
    }

    #[test]
    fn test_response_fields() {
        let now = chrono::Utc::now().naive_utc();
        let dob = chrono::NaiveDate::from_ymd_opt(1990, 5, 15).unwrap();

        let response = FamilyRelationshipResponse {
            id: 1,
            user_id: 10,
            related_user_id: None,
            related_person_name: Some("Jane Doe".to_string()),
            related_person_dob: Some(dob.format("%Y-%m-%d").to_string()),
            related_person_phone: Some("+1234567890".to_string()),
            related_person_email: Some("jane@example.com".to_string()),
            relationship_type: "spouse".to_string(),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(response.id, 1);
        assert_eq!(response.user_id, 10);
        assert_eq!(response.related_person_name.clone().unwrap(), "Jane Doe");
        assert_eq!(response.relationship_type, "spouse");
    }
}
