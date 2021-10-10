#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod context;
mod db;
mod error;
mod schema;
mod todo;

use context::ApiContext;
use rocket::{ignite, Route};

fn main() {
    let all_routes: Vec<Route> = Vec::new().into_iter().chain(todo::get_routes()).collect();

    ignite()
        .manage(ApiContext {
            pool: db::get_connection_pool(),
        })
        .mount("/api", all_routes)
        .launch();
}
