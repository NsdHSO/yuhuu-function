use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// User profile table - Contact and demographic information
/// Separated from core user table following Single Responsibility Principle
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "user_profiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    #[sea_orm(unique)]
    pub user_id: i64,

    // Contact Information
    pub middle_name: Option<String>,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,

    // Demographics
    pub date_of_birth: Option<Date>,
    pub gender: Option<String>, // Male, Female
    pub marital_status: Option<String>, // Single, Married, Widowed, Divorced
    pub occupation: Option<String>,
    pub nationality: Option<String>,

    // Emergency Contact
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub emergency_contact_relationship: Option<String>,

    // Profile
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,

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
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
