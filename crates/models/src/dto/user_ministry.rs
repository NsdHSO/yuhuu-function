use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "user_ministries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    pub user_id: i64,
    pub ministry_id: i64,
    pub position: Option<String>, // Leader, Member, Coordinator, etc.
    pub join_date: Date,
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,

    #[sea_orm(
        belongs_to = "super::ministry::Entity",
        from = "Column::MinistryId",
        to = "super::ministry::Column::Id"
    )]
    Ministry,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::ministry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ministry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
