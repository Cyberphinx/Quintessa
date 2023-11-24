use serde::{Deserialize, Serialize};

pub mod create;
pub mod save;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataEducation {
    pub data: ResponseEducation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseEducation {
    pub id: i32,
    pub resume_id: i32,
    pub school: String,
    pub degree: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateEducation {
    pub resume_id: i32,
    pub school: String,
    pub degree: String,
}
