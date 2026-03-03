use http_response::{CustomError, HttpCodeW};
use models::dto::{MembershipHistory, MembershipHistoryActiveModel};
use models::internal::{
    CreateMembershipHistoryRequest, MembershipHistoryResponse, UpdateMembershipHistoryRequest,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

pub struct MembershipHistoryService;

impl MembershipHistoryService {
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i64,
        request: CreateMembershipHistoryRequest,
    ) -> Result<MembershipHistoryResponse, CustomError> {
        let start_date = if let Some(date_str) = request.start_date {
            Some(
                chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
                    CustomError::new(
                        HttpCodeW::BadRequest,
                        "Invalid start_date format. Use YYYY-MM-DD".to_string(),
                    )
                })?,
            )
        } else {
            None
        };

        let end_date = if let Some(date_str) = request.end_date {
            Some(
                chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
                    CustomError::new(
                        HttpCodeW::BadRequest,
                        "Invalid end_date format. Use YYYY-MM-DD".to_string(),
                    )
                })?,
            )
        } else {
            None
        };

        let now = chrono::Utc::now().naive_utc();

        let new_membership = MembershipHistoryActiveModel {
            user_id: Set(user_id),
            church_name: Set(request.church_name),
            start_date: Set(start_date),
            end_date: Set(end_date),
            transfer_type: Set(request.transfer_type),
            previous_role: Set(request.previous_role),
            transfer_letter_received: Set(request.transfer_letter_received),
            notes: Set(request.notes),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let membership = new_membership.insert(db).await.map_err(|e| {
            if e.to_string().contains("duplicate key") || e.to_string().contains("idx_membership_history_active_unique") {
                CustomError::new(
                    HttpCodeW::Conflict,
                    "User already has an active membership. Please close the existing membership before adding a new one.".to_string(),
                )
            } else {
                CustomError::from(e)
            }
        })?;

        Ok(membership.into())
    }

    pub async fn list_by_user(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<MembershipHistoryResponse>, CustomError> {
        use models::dto::membership_history::Column;

        let memberships = MembershipHistory::find()
            .filter(Column::UserId.eq(user_id))
            .all(db)
            .await?;

        Ok(memberships.into_iter().map(|m| m.into()).collect())
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        user_id: i64,
        membership_id: i64,
    ) -> Result<MembershipHistoryResponse, CustomError> {
        use models::dto::membership_history::Column;

        let membership = MembershipHistory::find_by_id(membership_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Membership history not found".to_string())
            })?;

        Ok(membership.into())
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_id: i64,
        membership_id: i64,
        request: UpdateMembershipHistoryRequest,
    ) -> Result<MembershipHistoryResponse, CustomError> {
        use models::dto::membership_history::Column;

        let existing = MembershipHistory::find_by_id(membership_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Membership history not found".to_string())
            })?;

        let mut active: MembershipHistoryActiveModel = existing.into();

        if let Some(church_name) = request.church_name {
            active.church_name = Set(church_name);
        }
        if let Some(date_str) = request.start_date {
            let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    "Invalid start_date format. Use YYYY-MM-DD".to_string(),
                )
            })?;
            active.start_date = Set(Some(date));
        }
        if let Some(date_str) = request.end_date {
            let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    "Invalid end_date format. Use YYYY-MM-DD".to_string(),
                )
            })?;
            active.end_date = Set(Some(date));
        }
        if request.transfer_type.is_some() {
            active.transfer_type = Set(request.transfer_type);
        }
        if request.previous_role.is_some() {
            active.previous_role = Set(request.previous_role);
        }
        if request.transfer_letter_received.is_some() {
            active.transfer_letter_received = Set(request.transfer_letter_received);
        }
        if request.notes.is_some() {
            active.notes = Set(request.notes);
        }

        active.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated = active.update(db).await.map_err(|e| {
            if e.to_string().contains("duplicate key") || e.to_string().contains("idx_membership_history_active_unique") {
                CustomError::new(
                    HttpCodeW::Conflict,
                    "User already has an active membership. Please close the existing membership before adding a new one.".to_string(),
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
        membership_id: i64,
    ) -> Result<(), CustomError> {
        use models::dto::membership_history::Column;

        let membership = MembershipHistory::find_by_id(membership_id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Membership history not found".to_string())
            })?;

        let active: MembershipHistoryActiveModel = membership.into();
        active.delete(db).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_membership_request() {
        let request = CreateMembershipHistoryRequest {
            church_name: "Grace Community Church".to_string(),
            start_date: Some("2020-01-15".to_string()),
            end_date: None,
            transfer_type: Some("transfer_in".to_string()),
            previous_role: Some("Worship Team".to_string()),
            transfer_letter_received: Some(true),
            notes: Some("Transferred from sister church".to_string()),
        };

        assert_eq!(request.church_name, "Grace Community Church");
        assert_eq!(request.start_date.clone().unwrap(), "2020-01-15");
        assert!(request.end_date.is_none());
        assert_eq!(request.transfer_type.clone().unwrap(), "transfer_in");
    }

    #[test]
    fn test_create_current_membership() {
        let request = CreateMembershipHistoryRequest {
            church_name: "Current Church".to_string(),
            start_date: Some("2024-01-01".to_string()),
            end_date: None,
            transfer_type: Some("new_member".to_string()),
            previous_role: None,
            transfer_letter_received: None,
            notes: None,
        };

        assert_eq!(request.church_name, "Current Church");
        assert!(request.end_date.is_none());
        assert_eq!(request.transfer_type.clone().unwrap(), "new_member");
    }

    #[test]
    fn test_update_membership_request() {
        let request = UpdateMembershipHistoryRequest {
            church_name: Some("Updated Church Name".to_string()),
            start_date: None,
            end_date: Some("2023-12-31".to_string()),
            transfer_type: None,
            previous_role: Some("Elder".to_string()),
            transfer_letter_received: Some(true),
            notes: Some("Membership ended".to_string()),
        };

        assert_eq!(request.church_name.clone().unwrap(), "Updated Church Name");
        assert_eq!(request.end_date.clone().unwrap(), "2023-12-31");
        assert!(request.start_date.is_none());
    }

    #[test]
    fn test_response_fields() {
        let now = chrono::Utc::now().naive_utc();

        let response = MembershipHistoryResponse {
            id: 1,
            user_id: 10,
            church_name: "Grace Community Church".to_string(),
            start_date: Some("2020-01-15".to_string()),
            end_date: None,
            transfer_type: Some("transfer_in".to_string()),
            previous_role: Some("Worship Team".to_string()),
            transfer_letter_received: Some(true),
            notes: Some("Great experience".to_string()),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(response.id, 1);
        assert_eq!(response.church_name, "Grace Community Church");
        assert!(response.end_date.is_none());
        assert_eq!(response.transfer_type.clone().unwrap(), "transfer_in");
    }

    #[test]
    fn test_transfer_types() {
        let types = vec!["transfer_in", "transfer_out", "new_member", "restored"];

        for transfer_type in types {
            let request = CreateMembershipHistoryRequest {
                church_name: "Test Church".to_string(),
                start_date: None,
                end_date: None,
                transfer_type: Some(transfer_type.to_string()),
                previous_role: None,
                transfer_letter_received: None,
                notes: None,
            };
            assert_eq!(request.transfer_type.clone().unwrap(), transfer_type);
        }
    }

    #[test]
    fn test_closed_membership() {
        let now = chrono::Utc::now().naive_utc();

        let response = MembershipHistoryResponse {
            id: 2,
            user_id: 10,
            church_name: "Previous Church".to_string(),
            start_date: Some("2018-01-01".to_string()),
            end_date: Some("2022-12-31".to_string()),
            transfer_type: Some("transfer_out".to_string()),
            previous_role: Some("Member".to_string()),
            transfer_letter_received: Some(true),
            notes: Some("Relocated to new city".to_string()),
            created_at: now,
            updated_at: now,
        };

        assert!(response.end_date.is_some());
        assert_eq!(response.end_date.clone().unwrap(), "2022-12-31");
    }
}
