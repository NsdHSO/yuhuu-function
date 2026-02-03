use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), Givings::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Givings::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Givings::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Givings::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Givings::GivingType).string().not_null())
                    .col(ColumnDef::new(Givings::Amount).decimal().not_null())
                    .col(
                        ColumnDef::new(Givings::Currency)
                            .string()
                            .not_null()
                            .default("USD"),
                    )
                    .col(ColumnDef::new(Givings::GivingDate).date().not_null())
                    .col(ColumnDef::new(Givings::PaymentMethod).string().not_null())
                    .col(ColumnDef::new(Givings::ReferenceNumber).string())
                    .col(ColumnDef::new(Givings::ReceiptNumber).string())
                    .col(ColumnDef::new(Givings::FundCategory).string())
                    .col(
                        ColumnDef::new(Givings::IsRecurring)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Givings::RecurringFrequency).string())
                    .col(ColumnDef::new(Givings::VerifiedBy).big_integer())
                    .col(ColumnDef::new(Givings::VerifiedAt).timestamp())
                    .col(
                        ColumnDef::new(Givings::IsTaxDeductible)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Givings::Notes).text())
                    .col(
                        ColumnDef::new(Givings::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Givings::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_givings_user_id")
                            .from((Alias::new("church"), Givings::Table), Givings::UserId)
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_givings_verified_by")
                            .from((Alias::new("church"), Givings::Table), Givings::VerifiedBy)
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

        // Create index for faster lookups by date and type
        manager
            .create_index(
                Index::create()
                    .name("idx_givings_date_type")
                    .table((Alias::new("church"), Givings::Table))
                    .col(Givings::GivingDate)
                    .col(Givings::GivingType)
                    .to_owned(),
            )
            .await?;

        // Create index for receipt number lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_givings_receipt_number")
                    .table((Alias::new("church"), Givings::Table))
                    .col(Givings::ReceiptNumber)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), Givings::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Givings {
    Table,
    Id,
    Uuid,
    UserId,
    GivingType,
    Amount,
    Currency,
    GivingDate,
    PaymentMethod,
    ReferenceNumber,
    ReceiptNumber,
    FundCategory,
    IsRecurring,
    RecurringFrequency,
    VerifiedBy,
    VerifiedAt,
    IsTaxDeductible,
    Notes,
    CreatedAt,
    UpdatedAt,
}
