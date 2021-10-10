use r2d2;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use std::convert::From;

#[derive(Debug)]
pub struct AppError {
    code: u16,
    message: &'static str,
}

pub type AppResult<T> = Result<T, AppError>;

impl<'a> Responder<'a> for AppError {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Err(Status::new(self.code, self.message))
    }
}

impl From<r2d2::Error> for AppError {
    fn from(_: r2d2::Error) -> AppError {
        AppError {
            code: 500,
            message: "Error getting the db connection from the pool",
        }
    }
}
