use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "dinner_participants")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    pub dinner_id: i64,
    pub user_id: i64,
    pub notes: Option<String>,
    pub recorded_by: Option<i64>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dinner::Entity",
        from = "Column::DinnerId",
        to = "super::dinner::Column::Id"
    )]
    Dinner,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::RecordedBy",
        to = "super::user::Column::Id"
    )]
    RecordedByUser,
}

impl Related<super::dinner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dinner.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
