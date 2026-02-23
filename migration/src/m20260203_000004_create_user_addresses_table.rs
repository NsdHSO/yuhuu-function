use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), UserAddresses::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAddresses::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserAddresses::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserAddresses::UserId)
                            .big_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserAddresses::AddressLine1).string())
                    .col(ColumnDef::new(UserAddresses::AddressLine2).string())
                    .col(ColumnDef::new(UserAddresses::City).string())
                    .col(ColumnDef::new(UserAddresses::State).string())
                    .col(ColumnDef::new(UserAddresses::PostalCode).string())
                    .col(ColumnDef::new(UserAddresses::Country).string())
                    .col(
                        ColumnDef::new(UserAddresses::AddressType)
                            .string()
                            .not_null()
                            .default("Home"),
                    )
                    .col(
                        ColumnDef::new(UserAddresses::IsPrimary)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(UserAddresses::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserAddresses::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_addresses_user_id")
                            .from(
                                (Alias::new("church"), UserAddresses::Table),
                                UserAddresses::UserId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
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
                    .table((Alias::new("church"), UserAddresses::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserAddresses {
    Table,
    Id,
    Uuid,
    UserId,
    AddressLine1,
    AddressLine2,
    City,
    State,
    PostalCode,
    Country,
    AddressType,
    IsPrimary,
    CreatedAt,
    UpdatedAt,
}
