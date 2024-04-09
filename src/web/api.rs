use rocket::response::content;

use crate::resources::embedded::Components;

#[get("/todos")]
pub(crate) fn todo_list() -> content::RawHtml<String> {
    let content = Components::get("todo-list.html").unwrap();
    let string = String::from_utf8(content.data.into_owned()).unwrap();
    content::RawHtml(string)
}
