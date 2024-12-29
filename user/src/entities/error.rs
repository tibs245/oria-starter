use thiserror::Error;
#[derive(Error, Debug)]
pub enum UserError {
    #[error("Externals required services not accessible")]
    ServerError,
    #[error("Content already exists")]
    Duplicated,
}

