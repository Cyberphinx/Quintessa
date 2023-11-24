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
                    .table(Work::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Work::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Work::ResumeId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-work-resume_id")
                            .from(Work::Table, Work::ResumeId)
                            .to(Resume::Table, Resume::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Work::CompanyName).string().not_null())
                    .col(ColumnDef::new(Work::Position).string().not_null())
                    .col(ColumnDef::new(Work::Duration).string().not_null())
                    .col(ColumnDef::new(Work::Location).string().not_null())
                    .col(ColumnDef::new(Work::Projects).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Work::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Work {
    Table,
    Id,
    ResumeId,
    CompanyName,
    Position,
    Duration,
    Location,
    Projects,
}
