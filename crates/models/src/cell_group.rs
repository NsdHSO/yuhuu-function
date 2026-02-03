use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "cell_groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    pub name: String,
    pub description: Option<String>,
    pub zone_id: i64,
    pub leader_id: Option<i64>, // References users.id
    pub assistant_leader_ids: Option<String>, // JSON array of user IDs
    pub meeting_day: Option<String>, // Monday, Tuesday, etc.
    pub meeting_time: Option<Time>,
    pub meeting_location: Option<String>,
    pub max_capacity: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::zone::Entity",
        from = "Column::ZoneId",
        to = "super::zone::Column::Id"
    )]
    Zone,

    #[sea_orm(has_many = "super::user_membership::Entity")]
    UserMemberships,
}

impl Related<super::zone::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Zone.def()
    }
}

impl Related<super::user_membership::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserMemberships.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
