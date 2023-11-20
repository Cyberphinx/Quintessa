use domain::user;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::prelude::*;
use std::env;

use crate::utilities::hash::hash_password;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_admin_email = env::var("SEED_ADMIN_EMAIL").expect("Seed admin email not found!");
        let seed_admin_password =
            env::var("SEED_ADMIN_PASSWORD").expect("Seed admin password not found!");

        user::ActiveModel {
            email: Set(seed_admin_email),
            password: Set(hash_password(&seed_admin_password)
                .map_err(|_| sea_orm_migration::DbErr::RecordNotInserted)?),
            role: Set("admin".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        let seed_user_email = env::var("SEED_USER_EMAIL").expect("Seed user email not found!");
        let seed_user_password =
            env::var("SEED_USER_PASSWORD").expect("Seed user password not found!");

        user::ActiveModel {
            email: Set(seed_user_email),
            password: Set(hash_password(&seed_user_password)
                .map_err(|_| sea_orm_migration::DbErr::RecordNotInserted)?),
            role: Set("user".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_admin_email = env::var("SEED_ADMIN_EMAIL").expect("Seed admin email not found!");
        let seed_admin_password =
            env::var("SEED_ADMIN_PASSWORD").expect("Seed admin password not found!");

        user::ActiveModel {
            email: Set(seed_admin_email),
            password: Set(hash_password(&seed_admin_password)
                .map_err(|_| sea_orm_migration::DbErr::RecordNotInserted)?),
            role: Set("admin".to_owned()),
            ..Default::default()
        }
        .delete(db)
        .await?;

        let seed_user_email = env::var("SEED_USER_EMAIL").expect("Seed user email not found!");
        let seed_user_password =
            env::var("SEED_USER_PASSWORD").expect("Seed user password not found!");

        user::ActiveModel {
            email: Set(seed_user_email),
            password: Set(hash_password(&seed_user_password)
                .map_err(|_| sea_orm_migration::DbErr::RecordNotInserted)?),
            role: Set("user".to_owned()),
            ..Default::default()
        }
        .delete(db)
        .await?;

        Ok(())
    }
}
