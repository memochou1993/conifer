use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Record {
    pub id: String,
    pub url: String,
}
