use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "visit_assignments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    pub family_id: i64,
    pub assigned_to_user_id: i64,

    pub scheduled_date: Date,
    pub status: String,

    pub arrived_at: Option<DateTime>,
    pub arrived_latitude: Option<Decimal>,
    pub arrived_longitude: Option<Decimal>,

    pub completed_at: Option<DateTime>,
    pub notes: Option<String>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::visitable_family::Entity",
        from = "Column::FamilyId",
        to = "super::visitable_family::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    VisitableFamily,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AssignedToUserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    AssignedUser,
}

impl Related<super::visitable_family::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VisitableFamily.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssignedUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
