use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Resume::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Resume::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Resume::Name).string().not_null())
                    .col(ColumnDef::new(Resume::Email).string().not_null())
                    .col(ColumnDef::new(Resume::Mobile).string().not_null())
                    .col(ColumnDef::new(Resume::Website).string().not_null())
                    .col(ColumnDef::new(Resume::Birthdate).string().not_null())
                    .col(ColumnDef::new(Resume::Nationality).string().not_null())
                    .col(ColumnDef::new(Resume::TopSkills).string().not_null())
                    .col(ColumnDef::new(Resume::Languages).string().not_null())
                    .col(ColumnDef::new(Resume::Certifications).string().not_null())
                    .col(ColumnDef::new(Resume::Snippets).string().not_null())
                    .col(ColumnDef::new(Resume::JobTitle).string().not_null())
                    .col(ColumnDef::new(Resume::Address).string().not_null())
                    .col(ColumnDef::new(Resume::Summary).string().not_null())
                    .col(ColumnDef::new(Resume::Workshops).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Resume::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Resume {
    Table,
    Id,
    Name,
    Email,
    Mobile,
    Website,
    Birthdate,
    Nationality,
    TopSkills,
    Languages,
    Certifications,
    Snippets,
    JobTitle,
    Address,
    Summary,
    Workshops,
}
