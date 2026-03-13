use chrono::{NaiveDate, NaiveDateTime, Utc};
use http_response::{CustomError, HttpCodeW};
use models::dto::{User, UserProfile, VisitAssignment, VisitAssignmentActiveModel, VisitableFamily};
use models::internal::{
    AssignedUserBrief, CreateVisitAssignmentRequest, MarkArrivalRequest, MarkCompleteRequest,
    UpdateVisitAssignmentRequest, VisitAssignmentResponse, VisitableFamilyBrief,
};
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use std::str::FromStr;

pub struct VisitAssignmentService;

impl VisitAssignmentService {
    fn parse_date(date_str: &str) -> Result<NaiveDate, CustomError> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| CustomError::new(HttpCodeW::BadRequest, "Invalid date format".to_string()))
    }

    fn validate_future_date(date: NaiveDate) -> Result<(), CustomError> {
        let today = Utc::now().date_naive();
        if date < today {
            return Err(CustomError::new(HttpCodeW::BadRequest, "Date cannot be in past".to_string()));
        }
        Ok(())
    }

    fn convert_to_decimal(value: f64) -> Result<Decimal, CustomError> {
        Decimal::from_str(&value.to_string())
            .map_err(|_| CustomError::new(HttpCodeW::BadRequest, "Invalid coordinate".to_string()))
    }

    async fn check_active_assignments(db: &DatabaseConnection, family_id: i64) -> Result<(), CustomError> {
        use models::dto::visit_assignment::Column;
        let count = VisitAssignment::find()
            .filter(Column::FamilyId.eq(family_id))
            .filter(Column::Status.is_in(vec!["pending", "in_progress"]))
            .count(db).await?;
        if count > 0 {
            return Err(CustomError::new(HttpCodeW::Conflict, "Family has active assignment".to_string()));
        }
        Ok(())
    }

    async fn warn_pending_limit(db: &DatabaseConnection, user_id: i64) {
        use models::dto::visit_assignment::Column;
        if let Ok(count) = VisitAssignment::find()
            .filter(Column::AssignedToUserId.eq(user_id))
            .filter(Column::Status.eq("pending"))
            .count(db).await
        {
            if count > 5 {
                tracing::warn!("User {} has {} pending assignments", user_id, count);
            }
        }
    }

    pub async fn create(
        db: &DatabaseConnection,
        req: CreateVisitAssignmentRequest,
    ) -> Result<VisitAssignmentResponse, CustomError> {
        let date = Self::parse_date(&req.scheduled_date)?;
        Self::validate_future_date(date)?;
        Self::check_active_assignments(db, req.family_id).await?;
        Self::warn_pending_limit(db, req.assigned_to_user_id).await;

        let new_assignment = VisitAssignmentActiveModel {
            family_id: Set(req.family_id),
            assigned_to_user_id: Set(req.assigned_to_user_id),
            scheduled_date: Set(date),
            status: Set("pending".to_string()),
            notes: Set(req.notes),
            ..Default::default()
        };
        let result = new_assignment.insert(db).await?;
        Self::load_relations(db, result).await
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i64) -> Result<VisitAssignmentResponse, CustomError> {
        let assignment = VisitAssignment::find_by_id(id).one(db).await?.ok_or_else(|| {
            CustomError::new(HttpCodeW::NotFound, "Assignment not found".to_string())
        })?;
        Self::load_relations(db, assignment).await
    }

    pub async fn list_by_user(
        db: &DatabaseConnection,
        user_id: i64,
        status_filter: Option<String>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<VisitAssignmentResponse>, CustomError> {
        use models::dto::visit_assignment::Column;
        let mut query = VisitAssignment::find()
            .filter(Column::AssignedToUserId.eq(user_id))
            .order_by_desc(Column::ScheduledDate);
        if let Some(status) = status_filter {
            query = query.filter(Column::Status.eq(status));
        }
        let assignments = query.offset(offset).limit(limit).all(db).await?;
        let mut results = Vec::new();
        for a in assignments {
            results.push(Self::load_relations(db, a).await?);
        }
        Ok(results)
    }

    pub async fn list_all_admin(
        db: &DatabaseConnection,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<VisitAssignmentResponse>, CustomError> {
        use models::dto::visit_assignment::Column;
        let assignments = VisitAssignment::find()
            .order_by_desc(Column::ScheduledDate)
            .offset(offset).limit(limit).all(db).await?;
        let mut results = Vec::new();
        for a in assignments {
            results.push(Self::load_relations(db, a).await?);
        }
        Ok(results)
    }

    fn apply_admin_updates(
        mut active: VisitAssignmentActiveModel,
        req: UpdateVisitAssignmentRequest,
    ) -> Result<VisitAssignmentActiveModel, CustomError> {
        if let Some(d) = req.scheduled_date {
            active.scheduled_date = Set(Self::parse_date(&d)?);
        }
        if let Some(s) = req.status { active.status = Set(s); }
        if let Some(n) = req.notes { active.notes = Set(Some(n)); }
        Ok(active)
    }

    pub async fn update_admin(
        db: &DatabaseConnection,
        id: i64,
        req: UpdateVisitAssignmentRequest,
    ) -> Result<VisitAssignmentResponse, CustomError> {
        let assignment = VisitAssignment::find_by_id(id).one(db).await?.ok_or_else(|| {
            CustomError::new(HttpCodeW::NotFound, "Assignment not found".to_string())
        })?;
        let active: VisitAssignmentActiveModel = assignment.into();
        let updated_active = Self::apply_admin_updates(active, req)?;
        let updated = updated_active.update(db).await?;
        Self::load_relations(db, updated).await
    }

    pub async fn update_user_notes(
        db: &DatabaseConnection,
        id: i64,
        notes: Option<String>,
    ) -> Result<VisitAssignmentResponse, CustomError> {
        let assignment = VisitAssignment::find_by_id(id).one(db).await?.ok_or_else(|| {
            CustomError::new(HttpCodeW::NotFound, "Assignment not found".to_string())
        })?;
        let mut active: VisitAssignmentActiveModel = assignment.into();
        active.notes = Set(notes);
        let updated = active.update(db).await?;
        Self::load_relations(db, updated).await
    }

    fn validate_min_duration(arrived_at: NaiveDateTime) -> Result<(), CustomError> {
        let now = Utc::now().naive_utc();
        let duration = now.signed_duration_since(arrived_at);
        if duration.num_seconds() < 60 {
            return Err(CustomError::new(HttpCodeW::Conflict, "Minimum 1 minute required".to_string()));
        }
        Ok(())
    }

    pub async fn mark_arrival(
        db: &DatabaseConnection,
        id: i64,
        req: MarkArrivalRequest,
    ) -> Result<VisitAssignmentResponse, CustomError> {
        let assignment = VisitAssignment::find_by_id(id).one(db).await?.ok_or_else(|| {
            CustomError::new(HttpCodeW::NotFound, "Assignment not found".to_string())
        })?;
        if assignment.status != "pending" {
            return Err(CustomError::new(HttpCodeW::Conflict, "Must be pending".to_string()));
        }
        let mut active: VisitAssignmentActiveModel = assignment.into();
        active.status = Set("in_progress".to_string());
        active.arrived_at = Set(Some(Utc::now().naive_utc()));
        active.arrived_latitude = Set(Some(Self::convert_to_decimal(req.latitude)?));
        active.arrived_longitude = Set(Some(Self::convert_to_decimal(req.longitude)?));
        let updated = active.update(db).await?;
        Self::load_relations(db, updated).await
    }

    pub async fn mark_complete(
        db: &DatabaseConnection,
        id: i64,
        req: MarkCompleteRequest,
    ) -> Result<VisitAssignmentResponse, CustomError> {
        let assignment = VisitAssignment::find_by_id(id).one(db).await?.ok_or_else(|| {
            CustomError::new(HttpCodeW::NotFound, "Assignment not found".to_string())
        })?;
        if assignment.status != "in_progress" {
            return Err(CustomError::new(HttpCodeW::Conflict, "Must be in_progress".to_string()));
        }
        let arrived = assignment.arrived_at.ok_or_else(|| {
            CustomError::new(HttpCodeW::Conflict, "No arrival time".to_string())
        })?;
        Self::validate_min_duration(arrived)?;
        let mut active: VisitAssignmentActiveModel = assignment.into();
        active.status = Set("completed".to_string());
        active.completed_at = Set(Some(Utc::now().naive_utc()));
        if let Some(n) = req.notes { active.notes = Set(Some(n)); }
        let updated = active.update(db).await?;
        Self::load_relations(db, updated).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<(), CustomError> {
        let assignment = VisitAssignment::find_by_id(id).one(db).await?.ok_or_else(|| {
            CustomError::new(HttpCodeW::NotFound, "Assignment not found".to_string())
        })?;
        if !["pending", "cancelled"].contains(&assignment.status.as_str()) {
            return Err(CustomError::new(HttpCodeW::Conflict, "Only pending/cancelled can be deleted".to_string()));
        }
        let active: VisitAssignmentActiveModel = assignment.into();
        active.delete(db).await?;
        Ok(())
    }

    async fn load_relations(
        db: &DatabaseConnection,
        model: models::dto::VisitAssignmentModel,
    ) -> Result<VisitAssignmentResponse, CustomError> {
        use models::dto::user_profile::Column as ProfileColumn;
        let family = VisitableFamily::find_by_id(model.family_id).one(db).await?;
        let user = User::find_by_id(model.assigned_to_user_id).one(db).await?;
        let user_brief = if let Some(u) = user {
            let profile = UserProfile::find()
                .filter(ProfileColumn::UserId.eq(u.id))
                .one(db).await?;
            let name = profile.and_then(|p| p.middle_name).unwrap_or_else(|| u.auth_user_id.clone());
            Some(AssignedUserBrief { id: u.id, name })
        } else { None };
        let mut response = VisitAssignmentResponse::from(model);
        response.family = family.map(|f| VisitableFamilyBrief {
            id: f.id, family_name: f.family_name, address_street: f.address_street, address_city: f.address_city,
        });
        response.assigned_user = user_brief;
        Ok(response)
    }
}
