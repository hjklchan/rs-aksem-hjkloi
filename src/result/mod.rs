pub mod error;

use error::ApiError;

pub type OhMyResult<T> = Result<T, ApiError>;
