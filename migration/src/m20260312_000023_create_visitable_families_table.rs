use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), VisitableFamilies::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VisitableFamilies::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(VisitableFamilies::FamilyName).string().not_null())
                    .col(ColumnDef::new(VisitableFamilies::AddressStreet).string().not_null())
                    .col(ColumnDef::new(VisitableFamilies::AddressCity).string().not_null())
                    .col(ColumnDef::new(VisitableFamilies::AddressPostal).string())
                    .col(ColumnDef::new(VisitableFamilies::Latitude).decimal_len(10, 8))
                    .col(ColumnDef::new(VisitableFamilies::Longitude).decimal_len(11, 8))
                    .col(ColumnDef::new(VisitableFamilies::Phone).string())
                    .col(ColumnDef::new(VisitableFamilies::Notes).text())
                    .col(
                        ColumnDef::new(VisitableFamilies::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(VisitableFamilies::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_visitable_families_name")
                    .table((Alias::new("church"), VisitableFamilies::Table))
                    .col(VisitableFamilies::FamilyName)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_visitable_families_location")
                    .table((Alias::new("church"), VisitableFamilies::Table))
                    .col(VisitableFamilies::Latitude)
                    .col(VisitableFamilies::Longitude)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.visitable_families ADD CONSTRAINT unique_family_address UNIQUE (family_name, address_street, address_city)"
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), VisitableFamilies::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum VisitableFamilies {
    Table,
    Id,
    FamilyName,
    AddressStreet,
    AddressCity,
    AddressPostal,
    Latitude,
    Longitude,
    Phone,
    Notes,
    CreatedAt,
    UpdatedAt,
}
