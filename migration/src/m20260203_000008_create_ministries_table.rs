use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), Ministries::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ministries::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ministries::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Ministries::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Ministries::Description).text())
                    .col(ColumnDef::new(Ministries::Department).string())
                    .col(ColumnDef::new(Ministries::LeaderId).big_integer())
                    .col(ColumnDef::new(Ministries::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Ministries::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Ministries::UpdatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ministries_leader_id")
                            .from((Alias::new("church"), Ministries::Table), Ministries::LeaderId)
                            .to((Alias::new("church"), Alias::new("users")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table((Alias::new("church"), Ministries::Table)).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Ministries {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    Department,
    LeaderId,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
