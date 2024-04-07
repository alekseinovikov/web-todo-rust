#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use rocket::response::content;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/components/"]
struct Components;

#[derive(RustEmbed)]
#[folder = "static/public/"]
struct Public;

#[get("/api/todos")]
async fn todo_list() -> content::RawHtml<String> {
    let content = Components::get("todo-list.html").unwrap();
    let string = String::from_utf8(content.data.into_owned()).unwrap();
    content::RawHtml(string)
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/public/index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, todo_list])
        .mount("/", FileServer::from("static/public"))
}
