use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), Dinners::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dinners::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Dinners::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Dinners::DinnerDate)
                            .date()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Dinners::MealType).string().not_null())
                    .col(ColumnDef::new(Dinners::Description).text())
                    .col(ColumnDef::new(Dinners::RecordedBy).big_integer())
                    .col(
                        ColumnDef::new(Dinners::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Dinners::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dinners_recorded_by")
                            .from(
                                (Alias::new("church"), Dinners::Table),
                                Dinners::RecordedBy,
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
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_dinners_date_meal_type")
                    .table((Alias::new("church"), Dinners::Table))
                    .col(Dinners::DinnerDate)
                    .col(Dinners::MealType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), Dinners::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Dinners {
    Table,
    Id,
    Uuid,
    DinnerDate,
    MealType,
    Description,
    RecordedBy,
    CreatedAt,
    UpdatedAt,
}
