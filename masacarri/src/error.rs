use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

#[derive(Debug)]
pub enum AppError {
    UnspecifiedErr,
    StdErr(Box<dyn std::error::Error>),
    PublishableErr(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            AppError::UnspecifiedErr => write!(f, "unspecified error"),
            AppError::StdErr(e) => e.fmt(f),
            AppError::PublishableErr(s) => write!(f, "{}", s),
        }
    }
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> HttpResponse {
        let msg = if let AppError::PublishableErr(s) = self {
            s.clone()
        } else {
            "system error".to_string()
        };

        eprintln!("{}", self);
        HttpResponse::build(self.status_code()).json(ErrorMessage { message: msg })
    }
}

impl<T: std::error::Error + 'static> From<T> for AppError {
    fn from(e: T) -> Self {
        AppError::StdErr(Box::new(e))
    }
}

pub type AppResult<R> = actix_web::Result<R, AppError>;

#[derive(Debug, thiserror::Error)]
pub struct SimpleError(pub String);

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
