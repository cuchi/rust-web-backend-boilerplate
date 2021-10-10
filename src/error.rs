use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use std::convert::From;

#[derive(Debug, Default)]
pub struct ApiError {
    code: Option<u16>,
    message: &'static str,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl<'a> Responder<'a> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Err(Status::new(self.code.unwrap_or(500), self.message))
    }
}

impl From<R2d2Error> for ApiError {
    fn from(_: R2d2Error) -> ApiError {
        ApiError {
            message: "Error getting the db connection from the pool",
            ..ApiError::default()
        }
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::NotFound => ApiError {
                code: Some(404),
                message: "Not found!",
            },
            _ => ApiError {
                code: None,
                message: "Error when querying the database",
            },
        }
    }
}
