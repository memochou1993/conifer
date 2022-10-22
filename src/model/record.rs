use crate::schema::records;
use diesel::prelude::*;
use rocket::serde::Serialize;
use std::time::SystemTime;

#[derive(Insertable, Queryable, Serialize)]
pub struct Record {
    pub id: String,
    pub url: String,
    pub token: Option<String>,
    pub expired_at: SystemTime,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
