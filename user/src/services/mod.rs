use crate::datastore::UserDatastore;
use crate::views::user_payload::UserWithCredentialsPayload;
use auth_module::datastore::{AuthDatastore, TokenDatastore};
use auth_module::entities::UserCredentials;
use auth_module::services::{
    AuthService,
};
use auth_module::views::response::UserPrivateDetails;
use std::error::Error;

pub mod add_user;
pub mod get_user;

pub struct UserService<AuthDatastoreImpl: AuthDatastore, TokenDatastoreImpl: TokenDatastore, UserDatastoreImpl: UserDatastore> {
    user_datastore: UserDatastoreImpl,
    auth_service: AuthService<AuthDatastoreImpl, TokenDatastoreImpl>,
}

impl<
    AuthDatastoreImpl: AuthDatastore,
    TokenDatastoreImpl: TokenDatastore,
    UserDatastoreImpl: UserDatastore,
> UserService<AuthDatastoreImpl, TokenDatastoreImpl, UserDatastoreImpl>
{
    pub fn new(
        auth_service: AuthService<AuthDatastoreImpl, TokenDatastoreImpl>,
        user_datastore: UserDatastoreImpl,
    ) -> Self {
        Self {
            auth_service,
            user_datastore,
        }
    }
}

pub trait UserAddService {
    fn add_user(
        &self,
        user_payload: UserWithCredentialsPayload,
    ) -> impl std::future::Future<Output = Result<UserCredentials, Box<dyn Error + Send + Sync + 'static>>>;
}

pub trait UserGetService {
    fn get_user(
        &self,
        user_id: String,
    ) -> impl std::future::Future<
        Output = Result<UserPrivateDetails, Box<dyn Error + Send + Sync + 'static>>,
    >;
}
