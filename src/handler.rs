use crate::{
    repository,
    response::{RespGetRecord, RespGetRecords},
};
use rocket::{http::Status, serde::json::Json};

#[get("/records")]
pub fn get_records() -> Result<Json<RespGetRecords>, Status> {
    let conn = &mut repository::connect();
    let records = repository::get_records(conn);
    match records {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespGetRecords { data: r })),
            None => Err(Status::NotFound),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/records/<id>")]
pub fn get_record(id: &str) -> Result<Json<RespGetRecord>, Status> {
    let conn = &mut repository::connect();
    let record = repository::get_record(conn, id);
    match record {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespGetRecord { data: r })),
            None => Err(Status::NotFound),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
