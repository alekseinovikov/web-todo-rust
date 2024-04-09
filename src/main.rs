#[macro_use]
extern crate rocket;

use web::routes::{api_routes, root_routes};

mod web;
mod resources;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", root_routes())
        .mount("/api", api_routes())
}
