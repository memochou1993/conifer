#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    conifer::rocket()
}
