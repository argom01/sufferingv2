use actix_web::HttpResponse;
use actix_web::{error::BlockingError, http::StatusCode};
use prisma_client_rust::{query, query_core};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(query::Error),
    OperationCancelled,
    HashingError(argon2::password_hash::Error),
    TokenError(jwt_simple::Error),
    BadPassword,
    HeaderConversionError(actix_web::http::header::ToStrError),
    Unauthorized(String),
    Other,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraint"),
            AppError::RecordNotFound => write!(f, "This record does not exist"),
            AppError::OperationCancelled => write!(f, "The running operation was cancelled"),
            AppError::BadPassword => write!(f, "Wrong password"),
            AppError::Unauthorized(e) => write!(f, "Unauthorized: {}", e),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            _ => write!(f, "Internal server error"),
        }
    }
}

impl From<query::Error> for AppError {
    fn from(e: query::Error) -> Self {
        AppError::DatabaseError(e)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            _ => AppError::HashingError(e),
        }
    }
}

impl From<jwt_simple::Error> for AppError {
    fn from(e: jwt_simple::Error) -> Self {
        match e {
            _ => AppError::TokenError(e),
        }
    }
}

impl From<actix_web::http::header::ToStrError> for AppError {
    fn from(e: actix_web::http::header::ToStrError) -> Self {
        match e {
            _ => AppError::HeaderConversionError(e),
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            err: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::RecordAlreadyExists => StatusCode::BAD_REQUEST,
            AppError::RecordNotFound => StatusCode::NOT_FOUND,
            AppError::BadPassword => StatusCode::UNAUTHORIZED,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            &AppError::DatabaseError(_) => StatusCode::IM_A_TEAPOT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
