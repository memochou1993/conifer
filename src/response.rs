use crate::model::Record;
use diesel::Queryable;
use rocket::serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct RespGetRecords {
    pub data: Vec<Record>,
}

#[derive(Queryable, Serialize)]
pub struct RespGetRecord {
    pub data: Record,
}
