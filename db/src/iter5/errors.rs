use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TutorFrontError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error_message: String,
}

impl TutorFrontError {
    fn error_response(&self) -> String {
        match self {
            TutorFrontError::DBError(msg) => {
                println!("Database error occured: {:?}", msg);
                "Database error".into()
            }
            TutorFrontError::ActixError(msg) => {
                println!("Server error occured: {:?}", msg);
                "Internal server error".into()
            }
            TutorFrontError::NotFound(msg) => {
                println!("Not found error occured: {:?}", msg);
                msg.into()
            }
            TutorFrontError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for TutorFrontError {
    fn status_code(&self) -> StatusCode {
        match self {
            TutorFrontError::DBError(_msg) | TutorFrontError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            TutorFrontError::NotFound(_msg) => StatusCode::NOT_FOUND,
            TutorFrontError::InvalidInput(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for TutorFrontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for TutorFrontError {
    fn from(err: actix_web::error::Error) -> Self {
        TutorFrontError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for TutorFrontError {
    fn from(err: SQLxError) -> Self {
        TutorFrontError::DBError(err.to_string())
    }
}
