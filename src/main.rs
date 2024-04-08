#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::{Request, response};
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{ContentType, Status};
use rocket::http::uri::Segments;
use rocket::response::{content, Responder};
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

struct EmbeddedFile {
    data: Vec<u8>,
    content_type: ContentType,
}

impl<'r> Responder<'r, 'static> for EmbeddedFile {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        response::Response::build()
            .sized_body(self.data.len(), std::io::Cursor::new(self.data))
            .header(self.content_type)
            .ok()
    }
}

#[get("/")]
async fn index() -> content::RawHtml<String> {
    let content = Public::get("index.html").unwrap();
    let string = String::from_utf8(content.data.into_owned()).unwrap();
    content::RawHtml(string)
}

#[get("/<file..>")]
fn public_files(file: std::path::PathBuf) -> Result<EmbeddedFile, Status> {
    let path = file.to_str().unwrap_or_default();
    match Public::get(path) {
        Some(content) => {
            let content_type = ContentType::from_extension(
                file.extension().and_then(|ext| ext.to_str()).unwrap_or_default()
            ).unwrap_or(ContentType::Binary);

            Ok(EmbeddedFile {
                data: content.data.into_owned(),
                content_type,
            })
        }
        None => Err(Status::NotFound),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, public_files, todo_list])
}
