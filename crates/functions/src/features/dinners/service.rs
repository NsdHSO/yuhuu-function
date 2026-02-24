use http_response::{CustomError, HttpCodeW};
use models::dto::{DinnerActiveModel, DinnerParticipantActiveModel, Dinner, DinnerParticipant};
use models::internal::{
    AddParticipantRequest, CreateDinnerRequest, DinnerResponse, DinnerWithParticipantsResponse,
    ParticipantResponse,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde_json::json;

pub struct DinnerService;

impl DinnerService {
    pub async fn create_dinner(
        db: &DatabaseConnection,
        request: CreateDinnerRequest,
        recorded_by: Option<i64>,
    ) -> Result<DinnerResponse, CustomError> {
        let dinner_date = chrono::NaiveDate::parse_from_str(&request.dinner_date, "%Y-%m-%d")
            .map_err(|_| CustomError::new(HttpCodeW::BadRequest, "Invalid date format. Use YYYY-MM-DD".to_string()))?;

        let now = chrono::Utc::now().naive_utc();
        let uuid = uuid::Uuid::new_v4();

        let new_dinner = DinnerActiveModel {
            uuid: Set(uuid),
            dinner_date: Set(dinner_date),
            meal_type: Set(request.meal_type),
            description: Set(request.description),
            recorded_by: Set(recorded_by),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let dinner = new_dinner.insert(db).await?;

        Ok(DinnerResponse {
            id: dinner.id,
            uuid: dinner.uuid,
            dinner_date: dinner.dinner_date,
            meal_type: dinner.meal_type,
            description: dinner.description,
            recorded_by: dinner.recorded_by,
            created_at: dinner.created_at,
            updated_at: dinner.updated_at,
        })
    }

    pub async fn get_dinner(
        db: &DatabaseConnection,
        dinner_id: i64,
    ) -> Result<DinnerResponse, CustomError> {
        let dinner = Dinner::find_by_id(dinner_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Dinner not found".to_string()))?;

        Ok(DinnerResponse {
            id: dinner.id,
            uuid: dinner.uuid,
            dinner_date: dinner.dinner_date,
            meal_type: dinner.meal_type,
            description: dinner.description,
            recorded_by: dinner.recorded_by,
            created_at: dinner.created_at,
            updated_at: dinner.updated_at,
        })
    }

    pub async fn list_dinners(
        db: &DatabaseConnection,
        page: i64,
        limit: i64,
    ) -> Result<serde_json::Value, CustomError> {
        let page = if page < 1 { 1 } else { page };
        let limit = if (1..=100).contains(&limit) { limit } else { 20 };

        use models::dto::dinner::Column;
        let dinners: Vec<DinnerResponse> = Dinner::find()
            .order_by(Column::DinnerDate, sea_orm::Order::Desc)
            .paginate(db, limit as u64)
            .fetch_page((page - 1) as u64)
            .await?
            .into_iter()
            .map(|d| DinnerResponse {
                id: d.id,
                uuid: d.uuid,
                dinner_date: d.dinner_date,
                meal_type: d.meal_type,
                description: d.description,
                recorded_by: d.recorded_by,
                created_at: d.created_at,
                updated_at: d.updated_at,
            })
            .collect();

        let total = Dinner::find().count(db).await?;

        Ok(json!({
            "data": dinners,
            "pagination": {
                "page": page,
                "limit": limit,
                "total": total,
                "total_pages": (total as f64 / limit as f64).ceil() as i64
            }
        }))
    }

    pub async fn add_participant(
        db: &DatabaseConnection,
        dinner_id: i64,
        request: AddParticipantRequest,
        recorded_by: Option<i64>,
    ) -> Result<ParticipantResponse, CustomError> {
        let dinner = Dinner::find_by_id(dinner_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Dinner not found".to_string()))?;

        use models::dto::dinner_participant::Column;
        let existing = DinnerParticipant::find()
            .filter(Column::DinnerId.eq(dinner_id))
            .filter(Column::UserId.eq(request.user_id))
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(CustomError::new(
                HttpCodeW::Conflict,
                "User already recorded for this dinner".to_string(),
            ));
        }

        let now = chrono::Utc::now().naive_utc();
        let uuid = uuid::Uuid::new_v4();

        let new_participant = DinnerParticipantActiveModel {
            uuid: Set(uuid),
            dinner_id: Set(dinner.id),
            user_id: Set(request.user_id),
            notes: Set(request.notes),
            recorded_by: Set(recorded_by),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let participant = new_participant.insert(db).await?;

        Ok(ParticipantResponse {
            id: participant.id,
            uuid: participant.uuid,
            dinner_id: participant.dinner_id,
            user_id: participant.user_id,
            notes: participant.notes,
            recorded_by: participant.recorded_by,
            created_at: participant.created_at,
            updated_at: participant.updated_at,
        })
    }

    pub async fn get_dinner_with_participants(
        db: &DatabaseConnection,
        dinner_id: i64,
    ) -> Result<DinnerWithParticipantsResponse, CustomError> {
        let dinner = Dinner::find_by_id(dinner_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Dinner not found".to_string()))?;

        use models::dto::dinner_participant::Column;
        let participants = DinnerParticipant::find()
            .filter(Column::DinnerId.eq(dinner_id))
            .all(db)
            .await?;

        let dinner_response = DinnerResponse {
            id: dinner.id,
            uuid: dinner.uuid,
            dinner_date: dinner.dinner_date,
            meal_type: dinner.meal_type,
            description: dinner.description,
            recorded_by: dinner.recorded_by,
            created_at: dinner.created_at,
            updated_at: dinner.updated_at,
        };

        let participant_responses: Vec<ParticipantResponse> = participants
            .into_iter()
            .map(|p| ParticipantResponse {
                id: p.id,
                uuid: p.uuid,
                dinner_id: p.dinner_id,
                user_id: p.user_id,
                notes: p.notes,
                recorded_by: p.recorded_by,
                created_at: p.created_at,
                updated_at: p.updated_at,
            })
            .collect();

        Ok(DinnerWithParticipantsResponse {
            dinner: dinner_response,
            participants: participant_responses,
        })
    }

    pub async fn remove_participant(
        db: &DatabaseConnection,
        dinner_id: i64,
        participant_id: i64,
    ) -> Result<(), CustomError> {
        let participant = DinnerParticipant::find_by_id(participant_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Participant not found".to_string()))?;

        if participant.dinner_id != dinner_id {
            return Err(CustomError::new(
                HttpCodeW::BadRequest,
                "Participant does not belong to this dinner".to_string(),
            ));
        }

        let active_model: DinnerParticipantActiveModel = participant.into();
        active_model.delete(db).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_dinner_request_validation() {
        let request = CreateDinnerRequest {
            dinner_date: "2026-02-24".to_string(),
            meal_type: "Lunch".to_string(),
            description: Some("Sunday lunch after service".to_string()),
        };

        assert_eq!(request.dinner_date, "2026-02-24");
        assert_eq!(request.meal_type, "Lunch");
        assert_eq!(request.description.unwrap(), "Sunday lunch after service");
    }

    #[test]
    fn test_add_participant_request() {
        let request = AddParticipantRequest {
            user_id: 1,
            notes: Some("Vegetarian".to_string()),
        };

        assert_eq!(request.user_id, 1);
        assert_eq!(request.notes.unwrap(), "Vegetarian");
    }

    #[test]
    fn test_dinner_response_fields() {
        let now = chrono::Utc::now().naive_utc();
        let dinner_uuid = uuid::Uuid::new_v4();

        let response = DinnerResponse {
            id: 1,
            uuid: dinner_uuid,
            dinner_date: chrono::NaiveDate::from_ymd_opt(2026, 2, 24).unwrap(),
            meal_type: "Dinner".to_string(),
            description: Some("Test dinner".to_string()),
            recorded_by: Some(1),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(response.id, 1);
        assert_eq!(response.meal_type, "Dinner");
        assert!(response.description.is_some());
    }

    #[test]
    fn test_participant_response_fields() {
        let now = chrono::Utc::now().naive_utc();
        let participant_uuid = uuid::Uuid::new_v4();

        let response = ParticipantResponse {
            id: 1,
            uuid: participant_uuid,
            dinner_id: 1,
            user_id: 5,
            notes: None,
            recorded_by: Some(2),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(response.dinner_id, 1);
        assert_eq!(response.user_id, 5);
        assert!(response.notes.is_none());
    }

    #[test]
    fn test_dinner_with_participants_response() {
        let now = chrono::Utc::now().naive_utc();
        let dinner_uuid = uuid::Uuid::new_v4();
        let participant_uuid = uuid::Uuid::new_v4();

        let dinner = DinnerResponse {
            id: 1,
            uuid: dinner_uuid,
            dinner_date: chrono::NaiveDate::from_ymd_opt(2026, 2, 24).unwrap(),
            meal_type: "Lunch".to_string(),
            description: Some("Sunday lunch".to_string()),
            recorded_by: Some(1),
            created_at: now,
            updated_at: now,
        };

        let participants = vec![ParticipantResponse {
            id: 1,
            uuid: participant_uuid,
            dinner_id: 1,
            user_id: 5,
            notes: None,
            recorded_by: Some(1),
            created_at: now,
            updated_at: now,
        }];

        let response = DinnerWithParticipantsResponse {
            dinner,
            participants,
        };

        assert_eq!(response.dinner.id, 1);
        assert_eq!(response.participants.len(), 1);
        assert_eq!(response.participants[0].user_id, 5);
    }
}
