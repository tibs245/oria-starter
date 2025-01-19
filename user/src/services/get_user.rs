use std::error::Error;
use auth_module::services::AuthGetCredentialsService;
use crate::datastore::UserDatastore;
use crate::entities::error::UserError;
use crate::services::{UserGetService, UserService};
use crate::views::response::UserPrivateDetails;

impl<AuthServiceImpl, UserDatastoreImpl> UserGetService
for UserService<AuthServiceImpl, UserDatastoreImpl>
where
    AuthServiceImpl: AuthGetCredentialsService + 'static + Clone + Send + Sync,
    UserDatastoreImpl: UserDatastore + 'static + Clone + Send + Sync,
{
    async fn get_user(&self, username: &str) -> Result<UserPrivateDetails, Box<dyn Error + Send + Sync + 'static>> {
        let (user_credential, user_profile) = tokio::join!(
            self.auth_service.get_credentials_from_username(username),
            self.user_datastore.get_user_by_username(username)
        );

        let user_credential = user_credential?;
        let user_profile = user_profile?;

        match (user_credential, user_profile) {
            (Some(user_credential), Some(user_profile)) => {
                Ok(UserPrivateDetails::from_credential_and_user_profile(user_credential, user_profile))
            }
            (None, Some(_)) => { Err(Box::new(UserError::NoCredentials)) }
            (Some(_), None) => { Err(Box::new(UserError::NoProfile)) }
            (None, None) => { Err(Box::new(UserError::NoSuchProfile)) }
        }
    }
}