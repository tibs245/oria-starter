use crate::datastore::{UserDatastore};
use crate::views::user_payload::UserWithCredentialsPayload;
use auth_module::entities::UserCredentials;
use std::error::Error;
use auth_module::datastore::AuthDatastore;
use auth_module::services::{AuthCreateCredentialsService, AuthGetCredentialsService};
use crate::views::response::UserPrivateDetails;

pub mod add_user;
pub mod get_user;

pub struct UserService<AuthServiceImpl, UserDatastoreImpl: UserDatastore> {
    user_datastore: UserDatastoreImpl,
    auth_service: AuthServiceImpl,
}

impl<
    AuthServiceImpl,
    UserDatastoreImpl: UserDatastore,
> UserService<AuthServiceImpl, UserDatastoreImpl>
{
    pub fn new(
        auth_service: AuthServiceImpl,
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
    ) -> impl std::future::Future<Output=Result<UserCredentials, Box<dyn Error + Send + Sync + 'static>>>;
}

pub trait UserGetService {
    fn get_user(
        &self,
        username: &str,
    ) -> impl std::future::Future<
        Output=Result<UserPrivateDetails, Box<dyn Error + Send + Sync + 'static>>,
    >;
}
