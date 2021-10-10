#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod error;
mod context;
mod schema;
mod todo;

fn main() {
    let all_routes: Vec<rocket::Route> = Vec::new().into_iter().chain(todo::get_routes()).collect();

    rocket::ignite()
        .manage(context::AppContext {
            pool: db::get_connection_pool(),
        })
        .mount("/api", all_routes)
        .launch();
}
