use crate::errors::AppError;

pub(super) mod nouns;
pub(super) mod users;

type Result<T> = std::result::Result<T, AppError>;
