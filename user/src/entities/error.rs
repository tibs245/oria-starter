use thiserror::Error;
#[derive(Error, Debug)]
pub enum UserError {
    #[error("Externals required services not accessible")]
    ServerError,
    #[error("Content already exists")]
    Duplicated,
    #[error("No credentials related")]
    NoCredentials,
    #[error("No profile related")]
    NoProfile,
    #[error("No such profile")]
    NoSuchProfile,
}

