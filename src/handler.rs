use crate::{
    repository,
    request::ReqStoreRecord,
    response::{RespGetRecord, RespGetRecords, RespStoreRecord},
};
use rocket::{http::Status, response::Redirect, serde::json::Json};

#[get("/<id>")]
pub fn redirect(id: String) -> Result<Redirect, Status> {
    let conn = &mut repository::connect();
    let record = repository::get_record(conn, &id);
    match record {
        Ok(r) => match r {
            Some(r) => Ok(Redirect::to(r.url)),
            None => Err(Status::NotFound),
        },
        Err(e) => {
            print!("{}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/records")]
pub fn get_records() -> Result<Json<RespGetRecords>, Status> {
    let conn = &mut repository::connect();
    let records = repository::get_records(conn);
    match records {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespGetRecords { data: r })),
            None => Err(Status::NotFound),
        },
        Err(e) => {
            print!("{}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/records", format = "json", data = "<req>")]
pub fn store_record(req: Json<ReqStoreRecord>) -> Result<Json<RespStoreRecord>, Status> {
    let conn = &mut repository::connect();
    let record = repository::store_record(conn, &req.url);
    match record {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespStoreRecord { data: r })),
            None => Err(Status::NotFound),
        },
        Err(e) => {
            print!("{}", e);
            Err(Status::InternalServerError)
        }
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
        Err(e) => {
            print!("{}", e);
            Err(Status::InternalServerError)
        }
    }
}
