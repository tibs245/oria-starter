use thiserror::Error;
#[derive(Error, Debug, PartialEq)]
pub enum AuthError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Token create error")]
    TokenCreation,
    #[error("Token is invalid or expired")]
    InvalidToken,
    #[error("Externals required services not accessible")]
    ServerError,
    #[error("Content already exists")]
    Duplicated,
}

