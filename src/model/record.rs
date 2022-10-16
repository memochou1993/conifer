use crate::schema::records;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Serialize)]
pub struct Record {
    pub id: String,
    pub url: String,
    pub expired_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_at: SystemTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = records)]

pub struct NewRecord {
    pub id: String,
    pub url: String,
    pub token: Option<String>,
    pub expired_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_at: SystemTime,
}
