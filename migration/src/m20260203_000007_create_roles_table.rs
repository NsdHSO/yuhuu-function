use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), Roles::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Roles::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Roles::Description).text())
                    .col(ColumnDef::new(Roles::Level).integer().not_null().default(1))
                    .col(ColumnDef::new(Roles::Permissions).json())
                    .col(
                        ColumnDef::new(Roles::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Roles::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), Roles::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    Level,
    Permissions,
    CreatedAt,
    UpdatedAt,
}
