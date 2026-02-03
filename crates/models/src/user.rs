use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Core user table - Authentication and basic identity only
/// Following Single Responsibility Principle
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    // Core Identity
    pub first_name: String,
    pub last_name: String,

    // Authentication
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,

    // Account Status
    pub is_active: bool,
    pub is_email_verified: bool,
    pub email_verified_at: Option<DateTime>,

    // Timestamps
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub last_login: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::user_profile::Entity")]
    UserProfile,

    #[sea_orm(has_one = "super::user_membership::Entity")]
    UserMembership,

    #[sea_orm(has_one = "super::user_address::Entity")]
    UserAddress,

    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRoles,

    #[sea_orm(has_many = "super::user_ministry::Entity")]
    UserMinistries,

    #[sea_orm(has_many = "super::attendance::Entity")]
    Attendances,

    #[sea_orm(has_many = "super::giving::Entity")]
    Givings,
}

impl Related<super::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}

impl Related<super::user_membership::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserMembership.def()
    }
}

impl Related<super::user_address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAddress.def()
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl Related<super::user_ministry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserMinistries.def()
    }
}

impl Related<super::attendance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attendances.def()
    }
}

impl Related<super::giving::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Givings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
