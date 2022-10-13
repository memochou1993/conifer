#[macro_use]
extern crate rocket;
use crate::handler::{get_record, get_records, redirect, store_record};

mod handler;
mod model;
mod repository;
mod request;
mod response;
mod schema;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_records])
        .mount("/api", routes![store_record])
        .mount("/api", routes![get_record])
        .mount("/", routes![redirect])
}
