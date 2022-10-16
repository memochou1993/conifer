use crate::{
    database::{connect, record as model},
    request::record::ReqRecordStore,
    response::record::{RespRecordIndex, RespRecordShow, RespRecordStore},
};
use rocket::{
    http::Status,
    response::{status, Redirect},
    serde::json::Json,
};

#[get("/<id>")]
pub fn redirect(id: String) -> Result<Redirect, status::Custom<String>> {
    let conn = &mut connect();
    let record = model::get_by_id(conn, &id);
    match record {
        Ok(r) => match r {
            Some(r) => Ok(Redirect::to(r.url)),
            None => Err(status::Custom(Status::NotFound, "".to_string())),
        },
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}

#[get("/records")]
pub fn index() -> Result<Json<RespRecordIndex>, status::Custom<String>> {
    let conn = &mut connect();
    let records = model::get_all(conn);
    match records {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespRecordIndex { data: r })),
            None => Err(status::Custom(Status::NotFound, "".to_string())),
        },
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}

#[post("/records", format = "json", data = "<req>")]
pub fn store(req: Json<ReqRecordStore>) -> Result<Json<RespRecordStore>, status::Custom<String>> {
    let conn = &mut connect();
    let record = model::save(conn, &req.url);
    match record {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespRecordStore { data: r })),
            None => Err(status::Custom(Status::NotFound, "".to_string())),
        },
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}

#[get("/records/<id>")]
pub fn show(id: &str) -> Result<Json<RespRecordShow>, status::Custom<String>> {
    let conn = &mut connect();
    let record = model::get_by_id(conn, id);
    match record {
        Ok(r) => match r {
            Some(r) => Ok(Json(RespRecordShow { data: r })),
            None => Err(status::Custom(Status::NotFound, "".to_string())),
        },
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}

#[delete("/records/<id>")]
pub fn destroy(id: &str) -> Result<status::Custom<String>, status::Custom<String>> {
    let conn = &mut connect();
    let count = model::delete(conn, id);
    match count {
        Ok(c) => match c {
            c if c > 0 => Ok(status::Custom(Status::NoContent, "".to_string())),
            _ => Err(status::Custom(Status::NotFound, "".to_string())),
        },
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}
