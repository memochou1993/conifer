use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct RespRecord {
    pub id: String,
    pub url: String,
    pub expired_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct RespRecordIndex {
    pub data: Vec<RespRecord>,
}

#[derive(Serialize)]
pub struct RespRecordStore {
    pub data: RespRecord,
}

#[derive(Serialize)]
pub struct RespRecordShow {
    pub data: RespRecord,
}
