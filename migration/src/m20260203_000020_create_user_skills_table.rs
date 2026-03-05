use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS church.user_skills")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), UserSkills::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserSkills::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserSkills::UserId).big_integer().not_null())
                    .col(ColumnDef::new(UserSkills::SkillName).string().not_null())
                    .col(ColumnDef::new(UserSkills::SkillCategory).string())
                    .col(ColumnDef::new(UserSkills::ProficiencyLevel).string())
                    .col(ColumnDef::new(UserSkills::YearsOfExperience).integer())
                    .col(
                        ColumnDef::new(UserSkills::IsWillingToServe)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(UserSkills::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserSkills::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_skills_user_id")
                            .from(
                                (Alias::new("church"), UserSkills::Table),
                                UserSkills::UserId,
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
                    .name("idx_user_skills_user_skill_unique")
                    .table((Alias::new("church"), UserSkills::Table))
                    .col(UserSkills::UserId)
                    .col(UserSkills::SkillName)
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_skills_category")
                    .table((Alias::new("church"), UserSkills::Table))
                    .col(UserSkills::SkillCategory)
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
                    .table((Alias::new("church"), UserSkills::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserSkills {
    Table,
    Id,
    UserId,
    SkillName,
    SkillCategory,
    ProficiencyLevel,
    YearsOfExperience,
    IsWillingToServe,
    CreatedAt,
    UpdatedAt,
}
