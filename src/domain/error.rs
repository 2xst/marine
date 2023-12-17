pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("email is taken")]
    EmailTaken,
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid password")]
    InvalidPassword,
    #[error("validation error: {0}")]
    Validation(&'static str),
    #[error("an unexpected error occurred")]
    Unexpected(#[from] anyhow::Error),
}
