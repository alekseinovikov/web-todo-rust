#[macro_use]
extern crate rocket;
mod web;
mod resources;
mod desktop;

/*#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("template_dir", "."));

    rocket::custom(figment)
        .mount("/", root_routes())
        .mount("/api", api_routes())
        .attach(Template::custom(move |engines| {
            customize(&mut engines.tera);
        }))
}*/

pub fn main() -> iced::Result {
    desktop::ui::run()
}