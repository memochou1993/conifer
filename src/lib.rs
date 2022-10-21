#[macro_use]
extern crate rocket;
use crate::handler::record;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

mod database;
mod handler;
mod model;
mod request;
mod response;
mod schema;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![record::redirect])
        .mount("/api", routes![record::index])
        .mount("/api", routes![record::store])
        .mount("/api", routes![record::show])
        .mount("/api", routes![record::destroy])
        .attach(CORS)
}
