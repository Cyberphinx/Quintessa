use serde::{Deserialize, Serialize};

pub mod create;
pub mod save;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataWork {
    pub data: ResponseWork,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseWork {
    pub id: i32,
    pub resume_id: i32,
    pub company_name: String,
    pub position: String,
    pub duration: String,
    pub location: String,
    pub projects: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateWork {
    pub resume_id: i32,
    pub company_name: String,
    pub position: String,
    pub duration: String,
    pub location: String,
    pub projects: String,
}
