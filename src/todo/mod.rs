mod controller;
mod model;
mod service;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![controller::list]
}