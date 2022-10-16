use crate::schema::records;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub url: String,
    pub expired_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_at: SystemTime,
}
