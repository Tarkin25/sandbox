use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

error_chain! {
    errors {
        Sql(message: String) {
            description("Sql error"),
            display("Sql error: {}", message),
        }

        NotFound
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self.0 {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub trait OptionExt<T> {
    fn or_not_found(self) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_not_found(self) -> Result<T> {
        self.ok_or(Error::from_kind(ErrorKind::NotFound))
    }
}
