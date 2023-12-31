//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "resume")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub website: String,
    pub birthdate: String,
    pub nationality: String,
    pub top_skills: String,
    pub languages: String,
    pub certifications: String,
    pub snippets: String,
    pub job_title: String,
    pub address: String,
    pub summary: String,
    pub workshops: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::education::Entity")]
    Education,
    #[sea_orm(has_many = "super::work::Entity")]
    Work,
}

impl Related<super::education::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Education.def()
    }
}

impl Related<super::work::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Work.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
