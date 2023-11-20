pub mod create;
pub mod delete;
pub mod details;
pub mod edit;
pub mod list;
pub mod project_params;
pub mod save;

use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

use crate::media::ResponseMedia;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataProject {
    pub data: ResponseProject,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataProjectDto {
    pub data: ResponseProjectDto,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataProjects {
    pub data: Vec<ResponseProject>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataProjectDtos {
    pub data: Vec<ResponseProjectDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTotals {
    pub totals: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseProject {
    pub id: i32,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub name: Option<String>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    pub description: Option<String>,
    pub tasks: Option<String>,
    pub sector: Option<String>,
    pub location: Option<String>,
    pub address: Option<String>,
    pub client: Option<String>,
    pub start_date: Option<String>,
    pub completion_date: Option<String>,
    pub architect: Option<String>,
    pub main_contractor: Option<String>,
    pub project_manager: Option<String>,
    pub structural_engineer: Option<String>,
    pub services_engineer: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseProjectDto {
    pub id: i32,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub name: Option<String>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    pub description: Option<String>,
    pub tasks: Option<String>,
    pub sector: Option<String>,
    pub location: Option<String>,
    pub address: Option<String>,
    pub client: Option<String>,
    pub start_date: Option<String>,
    pub completion_date: Option<String>,
    pub architect: Option<String>,
    pub main_contractor: Option<String>,
    pub project_manager: Option<String>,
    pub structural_engineer: Option<String>,
    pub services_engineer: Option<String>,
    pub media: Option<Vec<ResponseMedia>>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateProject {
    pub name: Option<String>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    pub description: Option<String>,
    pub tasks: Option<String>,
    pub sector: Option<String>,
    pub location: Option<String>,
    pub address: Option<String>,
    pub client: Option<String>,
    pub start_date: Option<String>,
    pub completion_date: Option<String>,
    pub architect: Option<String>,
    pub main_contractor: Option<String>,
    pub project_manager: Option<String>,
    pub structural_engineer: Option<String>,
    pub services_engineer: Option<String>,
}
