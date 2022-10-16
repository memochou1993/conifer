#[macro_use]
extern crate rocket;
use crate::handler::record;

mod database;
mod handler;
mod model;
mod request;
mod response;
mod schema;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![record::redirect])
        .mount("/api", routes![record::index])
        .mount("/api", routes![record::store])
        .mount("/api", routes![record::show])
        .mount("/api", routes![record::destroy])
}
