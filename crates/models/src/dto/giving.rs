use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Giving/Tithes table - Track member giving (10% tithe, offerings, etc.)
/// Like Abraham gave to Melchizedek
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "givings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub uuid: Uuid,

    pub user_id: i64,

    // Giving Details
    pub giving_type: String, // Tithe (10%), Offering, First Fruit, Thanksgiving, Building Fund, Mission, etc.
    pub amount: Decimal,
    pub currency: String, // USD, EUR, etc.
    pub giving_date: Date,

    // Payment Information
    pub payment_method: String, // Cash, Check, Bank Transfer, Card, Mobile Money, etc.
    pub reference_number: Option<String>,
    pub receipt_number: Option<String>,

    // Categorization
    pub fund_category: Option<String>, // General, Building, Mission, Special Project
    pub is_recurring: bool,
    pub recurring_frequency: Option<String>, // Weekly, Monthly, Yearly

    // Verification
    pub verified_by: Option<i64>, // Admin/Pastor who verified (references users.id)
    pub verified_at: Option<DateTime>,
    pub is_tax_deductible: bool,

    pub notes: Option<String>,
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
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
