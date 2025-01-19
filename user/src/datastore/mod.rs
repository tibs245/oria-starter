use crate::entities::user::User;
#[cfg(test)]
use mockall::{automock};
use thiserror::Error;

pub mod mongo;

#[derive(Error, Debug)]
pub enum UserDatastoreError {
    #[error("Request not valid : {0}")]
    BadFormat(String),
    #[error("Unable processing request. Error with external services")]
    InternalError,
    #[error("The third-party service is not responding")]
    ProvidersError,
}

/// A trait representing a user data store for managing user information.
///
/// The `UserDataStore` trait provides an interface for manage user data
///
/// # Required Methods
///
/// Implementors of this trait must provide implementations for both
/// methods: `add_user` and `get_user_by_username`.
///
/// ## Errors
///
/// Both methods return a `Result` type. If an operation fails, an error of
/// type `UserDatastoreError` is returned.
///
/// # Attributes
///
/// The `#[cfg_attr(test, automock)]` attribute is applied to enable the
/// creation of mock implementations for testing purposes when the `test`
/// configuration is active. This is useful for unit testing components
/// that depend on `UserDataStore`.
///
/// ## Associated Types
///
/// - `User`: Represents the user entity that will be added or retrieved from the store.
/// - `UserDatastoreError`: Represents the possible errors that may occur during store operations.
#[cfg_attr(test, automock)]
pub trait UserDatastore {
    /// Adds a new user to the data store.
    ///
    /// # Arguments
    ///
    /// * `user` - The `User` instance to be added to the data store.
    ///
    /// # Errors
    ///
    /// If the user could not be added to the data store, an appropriate `UserDatastoreError` will be returned.
    fn add_user(&self, user: User) -> impl std::future::Future<Output=Result<User, UserDatastoreError>> + Send;

    /// Retrieves a user from the data store by their username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user to retrieve.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(user))` if the user was found.
    /// * `Ok(None)` if no user exists with the given username.
    /// * `Err(UserDatastoreError)` if an error occurred during retrieval.
    fn get_user_by_username(&self, username: &str) -> impl std::future::Future<Output=Result<Option<User>, UserDatastoreError>> + Send;
}