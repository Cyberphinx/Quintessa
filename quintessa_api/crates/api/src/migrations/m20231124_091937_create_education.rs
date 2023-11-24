use sea_orm_migration::prelude::*;

use super::m20231124_090934_create_resume::Resume;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Education::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Education::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Education::ResumeId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-work-resume_id")
                            .from(Education::Table, Education::ResumeId)
                            .to(Resume::Table, Resume::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Education::School).string().not_null())
                    .col(ColumnDef::new(Education::Degree).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Education::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Education {
    Table,
    Id,
    ResumeId,
    School,
    Degree,
}
