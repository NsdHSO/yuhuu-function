use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "visitable_families")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    pub family_name: String,
    pub address_street: String,
    pub address_city: String,
    pub address_postal: Option<String>,

    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,

    pub phone: Option<String>,
    pub notes: Option<String>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::visit_assignment::Entity")]
    VisitAssignments,
}

impl Related<super::visit_assignment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VisitAssignments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
