use crate::schema::records;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub url: String,
}
