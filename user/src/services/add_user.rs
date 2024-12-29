use crate::datastore::UserDatastore;
use crate::entities::error::UserError;
use crate::services::{UserAddService, UserService};
use crate::views::user_payload::{UserPayload, UserWithCredentialsPayload};
use auth_module::datastore::{AuthDatastore, TokenDatastore};
use auth_module::entities::UserCredentials;
use auth_module::services::AuthCreateCredentialsService;
use std::error::Error;

impl<AuthDatastoreImpl, TokenDatastoreImpl, UserDatastoreImpl> UserAddService
    for UserService<AuthDatastoreImpl, TokenDatastoreImpl, UserDatastoreImpl>
where
    AuthDatastoreImpl: AuthDatastore + 'static + Clone + Send + Sync,
    TokenDatastoreImpl: TokenDatastore + 'static + Clone + Send + Sync,
    UserDatastoreImpl: UserDatastore + 'static + Clone + Send + Sync,
{
    async fn add_user(
        &self,
        user_with_credential_payload: UserWithCredentialsPayload,
    ) -> Result<UserCredentials, Box<dyn Error + Send + Sync + 'static>> {
        if self
            .user_datastore
            .get_user_by_username(user_with_credential_payload.username.clone())
            .await?
            .is_some()
        {
            return Err(Box::new(UserError::Duplicated));
        }

        let user_credentials = self
            .auth_service
            .create_credentials(user_with_credential_payload.clone().into())
            .await?;
        self.user_datastore
            .add_user(user_with_credential_payload.into())
            .await?;

        Ok(user_credentials)
    }
}
