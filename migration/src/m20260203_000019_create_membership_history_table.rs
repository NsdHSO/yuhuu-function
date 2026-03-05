use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS church.membership_history")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), MembershipHistory::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MembershipHistory::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MembershipHistory::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MembershipHistory::ChurchName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MembershipHistory::StartDate).date())
                    .col(ColumnDef::new(MembershipHistory::EndDate).date())
                    .col(ColumnDef::new(MembershipHistory::TransferType).string())
                    .col(ColumnDef::new(MembershipHistory::PreviousRole).string())
                    .col(ColumnDef::new(MembershipHistory::TransferLetterReceived).boolean())
                    .col(ColumnDef::new(MembershipHistory::Notes).text())
                    .col(
                        ColumnDef::new(MembershipHistory::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MembershipHistory::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_membership_history_user_id")
                            .from(
                                (Alias::new("church"), MembershipHistory::Table),
                                MembershipHistory::UserId,
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
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_membership_history_user_end_date")
                    .table((Alias::new("church"), MembershipHistory::Table))
                    .col(MembershipHistory::UserId)
                    .col(MembershipHistory::EndDate)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE UNIQUE INDEX idx_membership_history_active_unique \
                 ON church.membership_history (user_id) \
                 WHERE end_date IS NULL",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), MembershipHistory::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum MembershipHistory {
    Table,
    Id,
    UserId,
    ChurchName,
    StartDate,
    EndDate,
    TransferType,
    PreviousRole,
    TransferLetterReceived,
    Notes,
    CreatedAt,
    UpdatedAt,
}
