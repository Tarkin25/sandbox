use std::fmt::{Display, Formatter};
use actix_web::{ResponseError, BaseHttpResponse, HttpResponse};
use actix_web::body::Body;
use sqlx::Error;

#[derive(Debug)]
pub enum AppError {
    Database,
    NotFound(i32),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AppError::*;

        match self {
            Database => write!(f, "Encountered a database error"),
            NotFound(id) => write!(f, "Resource with id \"{}\" not found", id)
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> BaseHttpResponse<Body> {
        use AppError::*;

        let response = match self {
            Database => HttpResponse::InternalServerError().body("Encountered a database error"),
            NotFound(id) => HttpResponse::NotFound().body(format!("Resource with id \"{}\" not found", id))
        };

        BaseHttpResponse::from(response)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: Error) -> Self {
        log::error!("Database error: {}", e);
        AppError::Database
    }
}