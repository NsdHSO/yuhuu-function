use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// User membership table - Church-specific membership information
/// Separated from user table following Single Responsibility Principle
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "user_memberships")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    #[sea_orm(unique)]
    pub user_id: i64,

    // Church Structure
    pub zone_id: Option<i64>,
    pub cell_group_id: Option<i64>,

    // Membership Status
    pub membership_status: String, // Visitor, New Convert, Member, Leader, Pastor, Inactive
    pub join_date: Option<Date>,
    pub membership_number: Option<String>,

    // Baptism Information
    pub is_water_baptized: bool,
    pub water_baptism_date: Option<Date>,
    pub water_baptism_location: Option<String>,

    pub is_holy_spirit_baptized: bool,
    pub holy_spirit_baptism_date: Option<Date>,

    // Spiritual Life
    pub spiritual_gifts: Option<String>, // JSON array: ["Teaching", "Prophecy", "Healing"]
    pub ministry_interests: Option<String>, // JSON array: ["Youth", "Music", "Evangelism"]
    pub salvation_testimony: Option<String>,

    // Previous Church
    pub previous_church_name: Option<String>,
    pub previous_church_location: Option<String>,
    pub transfer_letter_received: bool,

    // Notes
    pub notes: Option<String>,

    // Timestamps
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,

    #[sea_orm(
        belongs_to = "super::zone::Entity",
        from = "Column::ZoneId",
        to = "super::zone::Column::Id"
    )]
    Zone,

    #[sea_orm(
        belongs_to = "super::cell_group::Entity",
        from = "Column::CellGroupId",
        to = "super::cell_group::Column::Id"
    )]
    CellGroup,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::zone::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Zone.def()
    }
}

impl Related<super::cell_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CellGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
