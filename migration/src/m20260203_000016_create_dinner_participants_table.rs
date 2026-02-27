use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop table if exists (to handle schema changes)
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), DinnerParticipants::Table))
                    .to_owned(),
            )
            .await
            .ok(); // Ignore error if table doesn't exist

        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), DinnerParticipants::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DinnerParticipants::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DinnerParticipants::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(DinnerParticipants::DinnerId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DinnerParticipants::Username)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DinnerParticipants::Notes).text())
                    .col(
                        ColumnDef::new(DinnerParticipants::RecordedBy)
                            .big_integer(),
                    )
                    .col(
                        ColumnDef::new(DinnerParticipants::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DinnerParticipants::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dinner_participants_dinner_id")
                            .from(
                                (Alias::new("church"), DinnerParticipants::Table),
                                DinnerParticipants::DinnerId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("dinners")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dinner_participants_recorded_by")
                            .from(
                                (Alias::new("church"), DinnerParticipants::Table),
                                DinnerParticipants::RecordedBy,
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
                    .name("idx_dinner_participants_dinner_id")
                    .table((Alias::new("church"), DinnerParticipants::Table))
                    .col(DinnerParticipants::DinnerId)
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
                    .table((Alias::new("church"), DinnerParticipants::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum DinnerParticipants {
    Table,
    Id,
    Uuid,
    DinnerId,
    Username,
    Notes,
    RecordedBy,
    CreatedAt,
    UpdatedAt,
}
