use std::error::Error;
use auth_module::datastore::{AuthDatastore, TokenDatastore};
use auth_module::views::response::UserPrivateDetails;
use crate::datastore::UserDatastore;
use crate::services::{UserGetService, UserService};


impl<AuthDatastoreImpl, TokenDatastoreImpl, UserDatastoreImpl> UserGetService
for UserService<AuthDatastoreImpl, TokenDatastoreImpl, UserDatastoreImpl>
where AuthDatastoreImpl: AuthDatastore + 'static + Clone + Send + Sync,
      TokenDatastoreImpl: TokenDatastore + 'static + Clone + Send + Sync,
      UserDatastoreImpl: UserDatastore + 'static + Clone + Send + Sync
{
    async fn get_user(&self, user_id: String) -> Result<UserPrivateDetails, Box<dyn Error + Send + Sync + 'static>> {
        todo!()
    }
}