use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

pub mod save;

#[derive(Serialize, Deserialize)]
pub struct ResponseRefreshToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires: DateTimeWithTimeZone,
    pub revoked: Option<DateTimeWithTimeZone>,
}
