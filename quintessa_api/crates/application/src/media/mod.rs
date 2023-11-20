pub mod create;
pub mod delete;
pub mod edit;
pub mod list;
pub mod save;

use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseDataMedia {
    pub data: ResponseMedia,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataMedias {
    pub data: Vec<ResponseMedia>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseMedia {
    pub id: i32,
    pub project_id: i32,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub media_type: String,
    pub url: String,
    pub caption: Option<String>,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateMedia {
    pub project_id: i32,
    pub media_type: String,
    pub url: String,
    pub caption: Option<String>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestEditMedia {
    pub id: i32,
    pub project_id: i32,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub media_type: String,
    pub url: String,
    pub caption: Option<String>,
    pub description: String,
}
