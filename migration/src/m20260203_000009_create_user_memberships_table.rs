use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), UserMemberships::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserMemberships::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserMemberships::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserMemberships::UserId)
                            .big_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserMemberships::ZoneId).big_integer())
                    .col(ColumnDef::new(UserMemberships::CellGroupId).big_integer())
                    .col(
                        ColumnDef::new(UserMemberships::MembershipStatus)
                            .string()
                            .not_null()
                            .default("Visitor"),
                    )
                    .col(ColumnDef::new(UserMemberships::JoinDate).date())
                    .col(ColumnDef::new(UserMemberships::MembershipNumber).string())
                    .col(
                        ColumnDef::new(UserMemberships::IsWaterBaptized)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(UserMemberships::WaterBaptismDate).date())
                    .col(ColumnDef::new(UserMemberships::WaterBaptismLocation).string())
                    .col(
                        ColumnDef::new(UserMemberships::IsHolySpiritBaptized)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(UserMemberships::HolySpiritBaptismDate).date())
                    .col(ColumnDef::new(UserMemberships::SpiritualGifts).json())
                    .col(ColumnDef::new(UserMemberships::MinistryInterests).json())
                    .col(ColumnDef::new(UserMemberships::SalvationTestimony).text())
                    .col(ColumnDef::new(UserMemberships::PreviousChurchName).string())
                    .col(ColumnDef::new(UserMemberships::PreviousChurchLocation).string())
                    .col(
                        ColumnDef::new(UserMemberships::TransferLetterReceived)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(UserMemberships::Notes).text())
                    .col(
                        ColumnDef::new(UserMemberships::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserMemberships::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_memberships_user_id")
                            .from(
                                (Alias::new("church"), UserMemberships::Table),
                                UserMemberships::UserId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_memberships_zone_id")
                            .from(
                                (Alias::new("church"), UserMemberships::Table),
                                UserMemberships::ZoneId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("zones")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_memberships_cell_group_id")
                            .from(
                                (Alias::new("church"), UserMemberships::Table),
                                UserMemberships::CellGroupId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("cell_groups")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), UserMemberships::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserMemberships {
    Table,
    Id,
    Uuid,
    UserId,
    ZoneId,
    CellGroupId,
    MembershipStatus,
    JoinDate,
    MembershipNumber,
    IsWaterBaptized,
    WaterBaptismDate,
    WaterBaptismLocation,
    IsHolySpiritBaptized,
    HolySpiritBaptismDate,
    SpiritualGifts,
    MinistryInterests,
    SalvationTestimony,
    PreviousChurchName,
    PreviousChurchLocation,
    TransferLetterReceived,
    Notes,
    CreatedAt,
    UpdatedAt,
}
