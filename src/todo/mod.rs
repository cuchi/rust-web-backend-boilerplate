mod controller;
mod model;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![controller::list]
}