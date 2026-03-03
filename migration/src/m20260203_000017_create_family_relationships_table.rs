use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS church.family_relationships")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), FamilyRelationships::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FamilyRelationships::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::RelatedUserId)
                            .big_integer(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::RelatedPersonName)
                            .string(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::RelatedPersonDob)
                            .date(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::RelatedPersonPhone)
                            .string(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::RelatedPersonEmail)
                            .string(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::RelationshipType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationships::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_family_relationships_user_id")
                            .from(
                                (Alias::new("church"), FamilyRelationships::Table),
                                FamilyRelationships::UserId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_family_relationships_related_user_id")
                            .from(
                                (Alias::new("church"), FamilyRelationships::Table),
                                FamilyRelationships::RelatedUserId,
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
                    .name("idx_family_relationships_user_id")
                    .table((Alias::new("church"), FamilyRelationships::Table))
                    .col(FamilyRelationships::UserId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_family_relationships_related_user_id")
                    .table((Alias::new("church"), FamilyRelationships::Table))
                    .col(FamilyRelationships::RelatedUserId)
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
                    .table((Alias::new("church"), FamilyRelationships::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FamilyRelationships {
    Table,
    Id,
    UserId,
    RelatedUserId,
    RelatedPersonName,
    RelatedPersonDob,
    RelatedPersonPhone,
    RelatedPersonEmail,
    RelationshipType,
    CreatedAt,
    UpdatedAt,
}
