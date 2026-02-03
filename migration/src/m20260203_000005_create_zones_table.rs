use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), Zones::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Zones::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Zones::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Zones::Name).string().not_null())
                    .col(ColumnDef::new(Zones::Description).text())
                    .col(ColumnDef::new(Zones::ZoneLeaderId).big_integer())
                    .col(
                        ColumnDef::new(Zones::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Zones::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Zones::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_zones_leader_id")
                            .from((Alias::new("church"), Zones::Table), Zones::ZoneLeaderId)
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
                    .table((Alias::new("church"), Zones::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Zones {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    ZoneLeaderId,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
