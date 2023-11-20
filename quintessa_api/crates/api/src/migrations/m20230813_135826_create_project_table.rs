use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Project::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(ColumnDef::new(Project::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Project::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Project::Name).string().null())
                    .col(ColumnDef::new(Project::Category).string().null())
                    .col(ColumnDef::new(Project::SubCategory).string().null())
                    .col(ColumnDef::new(Project::Description).string().null())
                    .col(ColumnDef::new(Project::Tasks).string().null())
                    .col(ColumnDef::new(Project::Sector).string().null())
                    .col(ColumnDef::new(Project::Location).string().null())
                    .col(ColumnDef::new(Project::Address).string().null())
                    .col(ColumnDef::new(Project::Client).string().null())
                    .col(ColumnDef::new(Project::StartDate).string().null())
                    .col(ColumnDef::new(Project::CompletionDate).string().null())
                    .col(ColumnDef::new(Project::Architect).string().null())
                    .col(ColumnDef::new(Project::MainContractor).string().null())
                    .col(ColumnDef::new(Project::ProjectManager).string().null())
                    .col(ColumnDef::new(Project::StructuralEngineer).string().null())
                    .col(ColumnDef::new(Project::ServicesEngineer).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Project {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Name,
    Category,
    SubCategory,
    Description,
    Tasks,
    Sector,
    Location,
    Address,
    Client,
    StartDate,
    CompletionDate,
    Architect,
    MainContractor,
    ProjectManager,
    StructuralEngineer,
    ServicesEngineer,
}
