use actix_web::error::BlockingError;
use actix_web::web::HttpResponse;
use actix_web_httpauth::{extractors::basic::Config, headers::www_authenticate::Challenge};
use diesel::result::{
    DatabaseErrorKind::UniqueViolation,
    Error::{DatabaseError, NotFound},
};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCancelled,
    HashingError(argon2::password_hash::Error),
    TokenError(jwt_simple::Error),
    AuthError(String),
    HeaderConversionError(actix_web::http::header::ToStrError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraint"),
            AppError::RecordNotFound => write!(f, "This record does not exist"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperationCancelled => write!(f, "The running operation was cancelled"),
            AppError::HashingError(e) => write!(f, "Could not hash password: {:?}", e),
            AppError::TokenError(e) => write!(f, "Could not authenticate token: {:?}", e),
            AppError::AuthError(e) => write!(f, "Could not authenticate: {:?}", e),
            AppError::HeaderConversionError(e) => write!(f, "Could not convert type: {:?}", e),
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(e),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            _ => AppError::HashingError(e),
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> Self {
        match e {
            BlockingError::Error(inner) => inner,
            BlockingError::Canceled => AppError::OperationCancelled,
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
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            AppError::AuthError(_) => HttpResponse::BadRequest(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { err })
    }
}
