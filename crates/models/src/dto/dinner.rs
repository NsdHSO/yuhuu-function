use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "dinners")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    pub dinner_date: Date,
    pub meal_type: String,
    pub description: Option<String>,
    pub recorded_by: Option<i64>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::RecordedBy",
        to = "super::user::Column::Id"
    )]
    RecordedByUser,
    #[sea_orm(has_many = "super::dinner_participant::Entity")]
    DinnerParticipant,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecordedByUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
