use crate::model::record::Record;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct RespRecordIndex {
    pub data: Vec<Record>,
}

#[derive(Serialize)]
pub struct RespRecordStore {
    pub data: Record,
}

#[derive(Serialize)]
pub struct RespRecordShow {
    pub data: Record,
}
