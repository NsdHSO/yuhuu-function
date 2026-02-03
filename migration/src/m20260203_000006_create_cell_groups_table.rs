use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), CellGroups::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CellGroups::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CellGroups::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(CellGroups::Name).string().not_null())
                    .col(ColumnDef::new(CellGroups::Description).text())
                    .col(ColumnDef::new(CellGroups::ZoneId).big_integer().not_null())
                    .col(ColumnDef::new(CellGroups::LeaderId).big_integer())
                    .col(ColumnDef::new(CellGroups::AssistantLeaderIds).json())
                    .col(ColumnDef::new(CellGroups::MeetingDay).string())
                    .col(ColumnDef::new(CellGroups::MeetingTime).time())
                    .col(ColumnDef::new(CellGroups::MeetingLocation).string())
                    .col(ColumnDef::new(CellGroups::MaxCapacity).integer())
                    .col(
                        ColumnDef::new(CellGroups::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(CellGroups::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(CellGroups::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cell_groups_zone_id")
                            .from(
                                (Alias::new("church"), CellGroups::Table),
                                CellGroups::ZoneId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("zones")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cell_groups_leader_id")
                            .from(
                                (Alias::new("church"), CellGroups::Table),
                                CellGroups::LeaderId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("users")),
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
                    .table((Alias::new("church"), CellGroups::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum CellGroups {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    ZoneId,
    LeaderId,
    AssistantLeaderIds,
    MeetingDay,
    MeetingTime,
    MeetingLocation,
    MaxCapacity,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
