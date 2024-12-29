use std::error::Error;
use crate::datastore::{AuthDatastore, TokenDatastore};
#[cfg(test)]
use crate::datastore::{MockAuthDatastore, MockTokenDatastore};
use crate::entities::error::AuthError;
use crate::entities::{Token, UserCredentials};
use crate::utils::auth_claims::AuthClaims;
use crate::views::payload::{LoginPayload, RefreshTokenPayload};
use crate::views::response::AuthBody;

pub mod is_valid_credentials;
mod get_credentials_from_username;
pub(crate) mod create_credentials;
mod tokens;

pub trait AuthGetCredentialsService {
    fn get_credentials_from_username(&self, username: &str) -> impl std::future::Future<Output=Result<Option<UserCredentials>, Box<dyn Error + Send + Sync + 'static>>>;
}

pub trait AuthCreateCredentialsService {
    fn create_credentials(&self, auth_payload: LoginPayload) -> impl std::future::Future<Output=Result<UserCredentials, Box<dyn Error + Send + Sync + 'static>>>;
}

pub trait AuthValidCredentialsService {
    fn is_valid_credentials(&self, username: String, password: String) -> impl std::future::Future<Output=Result<UserCredentials, AuthError>>;
}


pub trait AuthTokensService {
    fn parse_auth_claims_from_refresh_payload(refresh_token_payload: RefreshTokenPayload) -> Result<AuthClaims, AuthError>;
    fn validate_token(&self, auth_claims: &AuthClaims) -> impl std::future::Future<Output=Result<Token, AuthError>>;
    fn try_get_user_token(&self, token: &Token) -> impl std::future::Future<Output=Result<UserCredentials, AuthError>>;
    fn generate_token(&self, user: &UserCredentials) -> impl std::future::Future<Output=Result<AuthBody, AuthError>>;
    fn refresh_tokens(&self, refresh_token_payload: RefreshTokenPayload) -> impl std::future::Future<Output=Result<AuthBody, AuthError>>;
}

#[derive(Clone)]
pub struct AuthService<AuthDatastoreImpl: AuthDatastore, TokenDatastoreImpl: TokenDatastore> {
    auth_datastore: AuthDatastoreImpl,
    token_datastore: TokenDatastoreImpl,
}

#[cfg(test)]
pub type MockAuthService = AuthService<MockAuthDatastore, MockTokenDatastore>;

#[cfg(test)]
impl MockAuthService {
    pub fn checkpoint(mut self) {
        self.auth_datastore.checkpoint();
        self.token_datastore.checkpoint();
    }
}

impl<AuthDatastoreImpl: AuthDatastore, TokenDatastoreImpl: TokenDatastore> AuthService<AuthDatastoreImpl, TokenDatastoreImpl> {
    pub fn new(auth_datastore: AuthDatastoreImpl, token_datastore: TokenDatastoreImpl) -> Self {
        Self {
            auth_datastore,
            token_datastore,
        }
    }
}
