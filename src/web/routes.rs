use rocket::Route;
use crate::web::api::todo_list;

use crate::web::static_files::{index, public_files};

pub fn root_routes() -> Vec<Route> {
    return routes![index, public_files];
}

pub fn api_routes() -> Vec<Route> {
    return routes![todo_list];
}
