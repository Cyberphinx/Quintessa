pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20230801_205119_seed_user;
mod m20230813_135826_create_project_table;
mod m20230901_202202_create_media_table;
mod m20231030_214811_create_refresh_token;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20230801_205119_seed_user::Migration),
            Box::new(m20230813_135826_create_project_table::Migration),
            Box::new(m20230901_202202_create_media_table::Migration),
            Box::new(m20231030_214811_create_refresh_token::Migration),
        ]
    }
}
