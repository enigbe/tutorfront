use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::convert::From;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl TutorError {
    fn error_response(&self) -> String {
        match self {
            TutorError::DBError(msg) => {
                println!("Database error occured: {:?}", msg);
                "Database error".into()
            }
            TutorError::ActixError(msg) => {
                println!("Server error occured: {:?}", msg);
                "Internal server error".into()
            }
            TutorError::TeraError(msg) => {
                println!("Error in rendering the template: {:?}", msg);
                msg.into()
            }
            TutorError::NotFound(msg) => {
                println!("Not found erroroccured: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for TutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            TutorError::DBError(_msg)
            | TutorError::ActixError(_msg)
            | TutorError::TeraError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            TutorError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for TutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}
impl From<actix_web::error::Error> for TutorError {
    fn from(err: actix_web::error::Error) -> Self {
        TutorError::ActixError(err.to_string())
    }
}
impl From<SQLxError> for TutorError {
    fn from(err: SQLxError) -> Self {
        TutorError::DBError(err.to_string())
    }
}
