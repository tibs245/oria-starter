use crate::datastore::{AuthDatastore, TokenDatastore};
use crate::entities::UserCredentials;
use crate::services::{AuthGetCredentialsService, AuthService};
use std::error::Error;

impl<AuthDatastoreImpl, TokenDatastoreImpl> AuthGetCredentialsService
for AuthService<AuthDatastoreImpl, TokenDatastoreImpl>
where
    AuthDatastoreImpl: AuthDatastore + 'static + Clone + Send + Sync,
    TokenDatastoreImpl: TokenDatastore + 'static + Clone + Send + Sync,
{
    async fn get_credentials_from_username(
        &self,
        username: &str,
    ) -> Result<Option<UserCredentials>, Box<dyn Error + Send + Sync + 'static>> {
        self.auth_datastore
            .get_user_by_username(&username)
            .await
            .map_err(|error| Box::from(error))
    }
}