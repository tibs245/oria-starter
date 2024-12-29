use crate::entities::{Token, UserCredentials};
#[cfg(test)]
use mockall::{automock, predicate::*};
use thiserror::Error;

pub mod memory;

pub mod mongo;

#[derive(Error, Debug)]
pub enum AuthDatastoreError {
    #[error("Request not valid : {0}")]
    BadFormat(String),
    #[error("Unable processing request. Error with external services")]
    InternalError,
    #[error("The third-party service is not responding")]
    ProvidersError
}

/// This trait defines the methods required for managing user authentication data in a datastore.
///
/// It is designed to be used with asynchronous operations and should be implemented for any datastore
/// that will be used to store and retrieve user credentials and tokens.
/// #[cfg(test)]
#[cfg_attr(test, automock)]
pub trait AuthDatastore {
    /// Adds a new user to the datastore with the provided credentials.
    ///
    /// # Arguments
    ///
    /// * `user` - A UserCredentials struct containing the username, hashed password of the new user and role.
    ///
    /// # Returns
    ///
    /// * `Result<UserCredentials, AuthDatastoreError>` - On success, returns the added UserCredentials. On failure,
    ///   returns an error of type AuthDatastoreError.
    fn add_user(&self, user: UserCredentials) -> impl std::future::Future<Output = Result<UserCredentials, AuthDatastoreError>> + Send;

    /// Retrieves a user from the datastore by their username.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice containing the username of the user to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<Option<UserCredentials>, AuthDatastoreError>` - On success, returns an Option containing the UserCredentials if the user is found,
    ///   or None if the user is not found. On failure, returns an error of type AuthDatastoreError.
    fn get_user_by_username(&self, username: &str) -> impl std::future::Future<Output = Result<Option<UserCredentials>, AuthDatastoreError>> + Send;
}


#[cfg(test)]
impl Clone for MockAuthDatastore {
    fn clone(&self) -> Self {
        todo!("It's a fake implementation of Clone")
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum TokenDatastoreError {
    #[error("Unable processing request. Error with external services")]
    InternalError,
    #[error("Unable create duplicated item")]
    ProvidersError
}
#[cfg_attr(test, automock)]
pub trait TokenDatastore {
    fn add_tokens(&self, token: Token) -> impl std::future::Future<Output = Result<Token, TokenDatastoreError>> + Send;
    fn get_token(&self, token_identifier: &str) -> impl std::future::Future<Output = Result<Option<Token>, TokenDatastoreError>> + Send;
    fn get_tokens_for_user(&self, username: &str) -> impl std::future::Future<Output = Result<Vec<Token>, TokenDatastoreError>> + Send;
    fn revoke_token(&self, token_identifier: &str) -> impl std::future::Future<Output = Result<(), TokenDatastoreError>> + Send;
}

#[cfg(test)]
impl Clone for MockTokenDatastore {
    fn clone(&self) -> Self {
        todo!("It's a fake implementation of Clone")
    }
}