use crate::{
    repository,
    response::{RespGetRecord, RespGetRecords},
};
use rocket::serde::json::Json;

#[get("/records")]
pub fn get_records() -> Json<RespGetRecords> {
    let conn = &mut repository::connect();
    let records = repository::get_records(conn);
    Json(RespGetRecords { data: records })
}

#[get("/records/<id>")]
pub fn get_record(id: &str) -> Json<RespGetRecord> {
    let conn = &mut repository::connect();
    let record = repository::get_record(conn, id);
    Json(RespGetRecord { data: record })
}
