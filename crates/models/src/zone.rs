use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "zones")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    pub name: String,
    pub description: Option<String>,
    pub zone_leader_id: Option<i64>, // References users.id
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_membership::Entity")]
    UserMemberships,

    #[sea_orm(has_many = "super::cell_group::Entity")]
    CellGroups,
}

impl Related<super::user_membership::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserMemberships.def()
    }
}

impl Related<super::cell_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CellGroups.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

