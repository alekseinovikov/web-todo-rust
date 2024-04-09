#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

use web::routes::{api_routes, root_routes};

use crate::web::templates::customize;

mod web;
mod resources;

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("template_dir", "."));

    rocket::custom(figment)
        .mount("/", root_routes())
        .mount("/api", api_routes())
        .attach(Template::custom(move |engines| {
            customize(&mut engines.tera);
        }))
}
