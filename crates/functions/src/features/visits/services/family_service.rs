use http_response::{CustomError, HttpCodeW};
use models::dto::{VisitableFamily, VisitableFamilyActiveModel, VisitAssignment};
use models::internal::{
    CreateVisitableFamilyRequest, UpdateVisitableFamilyRequest, VisitableFamilyResponse,
};
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use std::str::FromStr;

pub struct VisitableFamilyService;

impl VisitableFamilyService {
    fn validate_coordinates(lat: Option<f64>, lng: Option<f64>) -> Result<(), CustomError> {
        if let Some(latitude) = lat {
            if !(-90.0..=90.0).contains(&latitude) {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    "Latitude must be between -90 and 90".to_string(),
                ));
            }
        }
        if let Some(longitude) = lng {
            if !(-180.0..=180.0).contains(&longitude) {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    "Longitude must be between -180 and 180".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn convert_to_decimal(value: Option<f64>) -> Result<Option<Decimal>, CustomError> {
        value
            .map(|v| {
                Decimal::from_str(&v.to_string()).map_err(|_| {
                    CustomError::new(HttpCodeW::BadRequest, "Invalid decimal value".to_string())
                })
            })
            .transpose()
    }

    fn build_active_model(req: CreateVisitableFamilyRequest) -> Result<VisitableFamilyActiveModel, CustomError> {
        Ok(VisitableFamilyActiveModel {
            family_name: Set(req.family_name),
            address_street: Set(req.address_street),
            address_city: Set(req.address_city),
            address_postal: Set(req.address_postal),
            latitude: Set(Self::convert_to_decimal(req.latitude)?),
            longitude: Set(Self::convert_to_decimal(req.longitude)?),
            phone: Set(req.phone),
            notes: Set(req.notes),
            ..Default::default()
        })
    }

    fn handle_db_error(e: sea_orm::DbErr) -> CustomError {
        if e.to_string().contains("unique_family_address") {
            CustomError::new(HttpCodeW::Conflict, "Family already exists".to_string())
        } else {
            CustomError::from(e)
        }
    }

    pub async fn create(
        db: &DatabaseConnection,
        req: CreateVisitableFamilyRequest,
    ) -> Result<VisitableFamilyResponse, CustomError> {
        Self::validate_coordinates(req.latitude, req.longitude)?;
        let new_family = Self::build_active_model(req)?;
        let result = new_family.insert(db).await.map_err(Self::handle_db_error)?;
        Ok(VisitableFamilyResponse::from(result))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<VisitableFamilyResponse, CustomError> {
        let family = VisitableFamily::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Family not found".to_string())
            })?;

        Ok(VisitableFamilyResponse::from(family))
    }

    pub async fn list(
        db: &DatabaseConnection,
        limit: u64,
        offset: u64,
        search: Option<String>,
    ) -> Result<Vec<VisitableFamilyResponse>, CustomError> {
        use models::dto::visitable_family::Column;

        let mut query = VisitableFamily::find().order_by_asc(Column::FamilyName);

        if let Some(search_term) = search {
            let pattern = format!("%{}%", search_term);
            query = query.filter(
                Column::FamilyName
                    .like(&pattern)
                    .or(Column::AddressStreet.like(&pattern))
                    .or(Column::AddressCity.like(&pattern)),
            );
        }

        let families = query.offset(offset).limit(limit).all(db).await?;

        Ok(families.into_iter().map(VisitableFamilyResponse::from).collect())
    }

    fn apply_updates(
        mut active: VisitableFamilyActiveModel,
        req: UpdateVisitableFamilyRequest,
    ) -> Result<VisitableFamilyActiveModel, CustomError> {
        if let Some(v) = req.family_name { active.family_name = Set(v); }
        if let Some(v) = req.address_street { active.address_street = Set(v); }
        if let Some(v) = req.address_city { active.address_city = Set(v); }
        if let Some(v) = req.address_postal { active.address_postal = Set(Some(v)); }
        if req.latitude.is_some() { active.latitude = Set(Self::convert_to_decimal(req.latitude)?); }
        if req.longitude.is_some() { active.longitude = Set(Self::convert_to_decimal(req.longitude)?); }
        if let Some(v) = req.phone { active.phone = Set(Some(v)); }
        if let Some(v) = req.notes { active.notes = Set(Some(v)); }
        Ok(active)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        req: UpdateVisitableFamilyRequest,
    ) -> Result<VisitableFamilyResponse, CustomError> {
        Self::validate_coordinates(req.latitude, req.longitude)?;
        let family = VisitableFamily::find_by_id(id).one(db).await?.ok_or_else(|| {
            CustomError::new(HttpCodeW::NotFound, "Family not found".to_string())
        })?;
        let active: VisitableFamilyActiveModel = family.into();
        let updated_active = Self::apply_updates(active, req)?;
        let updated = updated_active.update(db).await.map_err(Self::handle_db_error)?;
        Ok(VisitableFamilyResponse::from(updated))
    }

    async fn has_active_assignments(db: &DatabaseConnection, family_id: i64) -> Result<bool, CustomError> {
        use models::dto::visit_assignment::Column;

        let count = VisitAssignment::find()
            .filter(Column::FamilyId.eq(family_id))
            .filter(Column::Status.is_in(vec!["pending", "in_progress"]))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<(), CustomError> {
        if Self::has_active_assignments(db, id).await? {
            return Err(CustomError::new(
                HttpCodeW::Conflict,
                "Cannot delete family with active assignments".to_string(),
            ));
        }

        let family = VisitableFamily::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Family not found".to_string())
            })?;

        let active: VisitableFamilyActiveModel = family.into();
        active.delete(db).await?;

        Ok(())
    }
}
