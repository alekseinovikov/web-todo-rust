use rocket::response::content;
use rocket_dyn_templates::{context, Template};

use crate::resources::embedded::Components;

#[get("/todos")]
pub(crate) fn todo_list() -> content::RawHtml<String> {
    let content = Components::get("todo-list.html").unwrap();
    let string = String::from_utf8(content.data.into_owned()).unwrap();
    content::RawHtml(string)
}

#[get("/hello")]
pub(crate) fn hello() -> Template {
    Template::render("todo/list.html.tera", context! {
        name: "EBANUSHKA!",
    })
}