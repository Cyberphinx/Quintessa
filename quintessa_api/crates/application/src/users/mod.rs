use serde::{Deserialize, Serialize};

pub mod create;
pub mod details;
pub mod edit;
pub mod save;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    pub data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub email: String,
    pub role: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseCreateUser {
    pub id: i32,
    pub email: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestEditRole {
    pub email: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseEditRole {
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth0Request {
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
    pub grant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth0Response {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}
