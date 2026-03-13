use crate::dto::VisitableFamilyModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FamilyListQuery {
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[serde(default)]
    pub offset: u64,
    pub search: Option<String>,
}

fn default_limit() -> u64 {
    50
}

#[derive(Debug, Deserialize)]
pub struct CreateVisitableFamilyRequest {
    pub family_name: String,
    pub address_street: String,
    pub address_city: String,
    pub address_postal: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub phone: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVisitableFamilyRequest {
    pub family_name: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_postal: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub phone: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VisitableFamilyResponse {
    pub id: i64,
    pub family_name: String,
    pub address_street: String,
    pub address_city: String,
    pub address_postal: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub phone: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<VisitableFamilyModel> for VisitableFamilyResponse {
    fn from(model: VisitableFamilyModel) -> Self {
        VisitableFamilyResponse {
            id: model.id,
            family_name: model.family_name,
            address_street: model.address_street,
            address_city: model.address_city,
            address_postal: model.address_postal,
            latitude: model.latitude.and_then(|d| d.to_string().parse().ok()),
            longitude: model.longitude.and_then(|d| d.to_string().parse().ok()),
            phone: model.phone,
            notes: model.notes,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
