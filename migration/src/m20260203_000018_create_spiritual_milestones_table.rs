use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS church.spiritual_milestones")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), SpiritualMilestones::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SpiritualMilestones::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SpiritualMilestones::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SpiritualMilestones::MilestoneType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SpiritualMilestones::MilestoneDate).date())
                    .col(ColumnDef::new(SpiritualMilestones::Location).string())
                    .col(ColumnDef::new(SpiritualMilestones::Officiant).string())
                    .col(ColumnDef::new(SpiritualMilestones::Notes).text())
                    .col(
                        ColumnDef::new(SpiritualMilestones::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SpiritualMilestones::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_spiritual_milestones_user_id")
                            .from(
                                (Alias::new("church"), SpiritualMilestones::Table),
                                SpiritualMilestones::UserId,
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
                    .name("idx_spiritual_milestones_user_milestone_unique")
                    .table((Alias::new("church"), SpiritualMilestones::Table))
                    .col(SpiritualMilestones::UserId)
                    .col(SpiritualMilestones::MilestoneType)
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), SpiritualMilestones::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum SpiritualMilestones {
    Table,
    Id,
    UserId,
    MilestoneType,
    MilestoneDate,
    Location,
    Officiant,
    Notes,
    CreatedAt,
    UpdatedAt,
}
