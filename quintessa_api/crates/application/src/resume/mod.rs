use serde::{Deserialize, Serialize};

use crate::{educations::ResponseEducation, works::ResponseWork};

pub mod create;
pub mod delete;
pub mod details;
pub mod edit;
pub mod save;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataResumeDto {
    pub data: ResponseResumeDto,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataResume {
    pub data: ResponseResume,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseResume {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseResumeDto {
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
    pub works: Vec<ResponseWork>,
    pub educations: Vec<ResponseEducation>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateResume {
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
