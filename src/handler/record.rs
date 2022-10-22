use crate::{
    database::{connect, record as model},
    request::record::ReqRecordStore,
    response::record::{RespRecord, RespRecordIndex, RespRecordShow, RespRecordStore},
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
        Some(r) => Ok(Redirect::to(r.url)),
        None => Err(status::Custom(Status::NotFound, "".to_string())),
    }
}

#[get("/records")]
pub fn index() -> Result<status::Custom<Json<RespRecordIndex>>, status::Custom<String>> {
    let conn = &mut connect();
    let records = model::get_all(conn);
    match records {
        Some(r) => {
            let data = r
                .iter()
                .map(|r| RespRecord {
                    id: r.id.clone(),
                    url: r.url.clone(),
                    expired_at: r.expired_at.into(),
                    created_at: r.expired_at.into(),
                    updated_at: r.expired_at.into(),
                })
                .collect();
            Ok(status::Custom(Status::Ok, Json(RespRecordIndex { data })))
        }
        None => Err(status::Custom(Status::NotFound, "".to_string())),
    }
}

#[post("/records", format = "json", data = "<req>")]
pub fn store(
    req: Json<ReqRecordStore>,
) -> Result<status::Custom<Json<RespRecordStore>>, status::Custom<String>> {
    let conn = &mut connect();
    let record = model::save(conn, &req.url, &req.token);
    match record {
        Some(r) => {
            let data = RespRecord {
                id: r.id.clone(),
                url: r.url.clone(),
                expired_at: r.expired_at.into(),
                created_at: r.expired_at.into(),
                updated_at: r.expired_at.into(),
            };
            Ok(status::Custom(
                Status::Created,
                Json(RespRecordStore { data }),
            ))
        }
        None => Err(status::Custom(Status::NotFound, "".to_string())),
    }
}

#[get("/records/<id>")]
pub fn show(id: &str) -> Result<status::Custom<Json<RespRecordShow>>, status::Custom<String>> {
    let conn = &mut connect();
    let record = model::get_by_id(conn, id);
    match record {
        Some(r) => {
            let data = RespRecord {
                id: r.id.clone(),
                url: r.url.clone(),
                expired_at: r.expired_at.into(),
                created_at: r.expired_at.into(),
                updated_at: r.expired_at.into(),
            };
            Ok(status::Custom(Status::Ok, Json(RespRecordShow { data })))
        }
        None => Err(status::Custom(Status::NotFound, "".to_string())),
    }
}

#[delete("/records/<id>?<token>")]
pub fn destroy(id: &str, token: &str) -> status::Custom<String> {
    let conn = &mut connect();
    let count = model::delete(conn, id, token);
    match count {
        Ok(c) => match c {
            c if c > 0 => status::Custom(Status::NoContent, "".to_string()),
            _ => status::Custom(Status::NotFound, "".to_string()),
        },
        Err(e) => status::Custom(Status::InternalServerError, e.to_string()),
    }
}
