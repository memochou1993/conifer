use crate::{
    database::{connect, record as model},
    request::record::ReqRecordStore,
    response::record::{RespRecordIndex, RespRecordShow, RespRecordStore},
};
use rocket::{http::Status, response::Redirect, serde::json::Json};

#[get("/<id>")]
pub fn redirect(id: String) -> Result<Redirect, Status> {
    let conn = &mut connect();
    let record = model::get_by_id(conn, &id);
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
pub fn index() -> Result<Json<RespRecordIndex>, Status> {
    let conn = &mut connect();
    let records = model::get_all(conn);
    match records {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespRecordIndex { data: r })),
            None => Err(Status::NotFound),
        },
        Err(e) => {
            print!("{}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/records", format = "json", data = "<req>")]
pub fn store(req: Json<ReqRecordStore>) -> Result<Json<RespRecordStore>, Status> {
    let conn = &mut connect();
    let record = model::save(conn, &req.url);
    match record {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespRecordStore { data: r })),
            None => Err(Status::NotFound),
        },
        Err(e) => {
            print!("{}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/records/<id>")]
pub fn show(id: &str) -> Result<Json<RespRecordShow>, Status> {
    let conn = &mut connect();
    let record = model::get_by_id(conn, id);
    match record {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespRecordShow { data: r })),
            None => Err(Status::NotFound),
        },
        Err(e) => {
            print!("{}", e);
            Err(Status::InternalServerError)
        }
    }
}
