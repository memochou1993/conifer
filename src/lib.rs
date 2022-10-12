#[macro_use]
extern crate rocket;
use crate::handler::{get_record, get_records};

mod handler;
mod model;
mod repository;
mod response;
mod schema;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_records])
        .mount("/api", routes![get_record])
}
