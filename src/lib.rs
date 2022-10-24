#[macro_use]
extern crate rocket;
use crate::fairing::cors::CORS;
use crate::handler::record;

mod database;
mod fairing;
mod handler;
mod model;
mod request;
mod response;
mod schema;

#[options("/<_..>")]
fn options() {}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![record::redirect, options])
        .mount("/api", routes![record::index])
        .mount("/api", routes![record::store])
        .mount("/api", routes![record::show])
        .mount("/api", routes![record::destroy])
        .attach(CORS)
}
