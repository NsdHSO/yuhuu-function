use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "ministries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    #[sea_orm(unique)]
    pub name: String, // Worship, Ushering, Media, Children, Youth, etc.
    pub description: Option<String>,
    pub department: Option<String>,
    pub leader_id: Option<i64>, // References users.id
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_ministry::Entity")]
    UserMinistries,
}

impl Related<super::user_ministry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserMinistries.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
