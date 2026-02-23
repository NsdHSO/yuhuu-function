use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Bridge table linking auth server users to church database
/// Auth server handles: authentication, email, password, name, verification
/// Church system handles: profiles, memberships, attendance, giving, ministries
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    /// Foreign key to auth server user (UUID from auth server)
    #[sea_orm(unique)]
    pub auth_user_id: String,

    // Timestamps
    pub created_at: DateTime,
    pub updated_at: DateTime,
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
