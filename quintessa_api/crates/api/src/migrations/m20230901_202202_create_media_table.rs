use sea_orm_migration::prelude::*;

use super::m20230813_135826_create_project_table::Project;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Media::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Media::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Media::ProjectId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-media-project_id")
                            .from(Media::Table, Media::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Media::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(ColumnDef::new(Media::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Media::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Media::MediaType).string().not_null())
                    .col(ColumnDef::new(Media::Url).string().not_null())
                    .col(ColumnDef::new(Media::Caption).string().null())
                    .col(ColumnDef::new(Media::Description).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Media::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Media {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    ProjectId,
    MediaType,
    Url,
    Caption,
    Description,
}
