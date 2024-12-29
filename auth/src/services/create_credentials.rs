use crate::datastore::{AuthDatastore, TokenDatastore};
use crate::entities::error::AuthError;
use crate::entities::UserCredentials;
use crate::services::{AuthCreateCredentialsService, AuthService};
use crate::views::payload::LoginPayload;
use std::error::Error;

impl<AuthDatastoreImpl, TokenDatastoreImpl> AuthCreateCredentialsService
    for AuthService<AuthDatastoreImpl, TokenDatastoreImpl>
where
    AuthDatastoreImpl: AuthDatastore + 'static + Clone + Send + Sync,
    TokenDatastoreImpl: TokenDatastore + 'static + Clone + Send + Sync,
{
    async fn create_credentials(
        &self,
        auth_payload: LoginPayload,
    ) -> Result<UserCredentials, Box<dyn Error + Send + Sync + 'static>> {
        if self
            .auth_datastore
            .get_user_by_username(&auth_payload.username)
            .await?
            .is_some()
        {
            return Err(Box::new(AuthError::Duplicated));
        }

        self.auth_datastore
            .add_user(auth_payload.into())
            .await
            .map_err(|error| Box::from(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datastore::{MockAuthDatastore, MockTokenDatastore};
    use crate::services::MockAuthService;
    use fake::{Fake, Faker};
    use mongodb::bson::oid::ObjectId;
    use std::future;

    #[tokio::test]
    async fn test_create_credential_model() {
        let mut mock = MockAuthDatastore::new();

        mock.expect_get_user_by_username()
            .times(1)
            .returning(|_username| Box::pin(future::ready(Ok(None))));

        mock.expect_add_user()
            .times(1)
            .returning(|user_credentials: UserCredentials| {
                Box::pin(future::ready(Ok(UserCredentials {
                    id: Some(ObjectId::new()),
                    ..user_credentials
                })))
            });

        let login_payload: LoginPayload = Faker.fake();
        let auth_service = MockAuthService::new(mock, MockTokenDatastore::new());
        let result = auth_service
            .create_credentials(login_payload.clone())
            .await
            .expect("Unable create user credential with mock");

        auth_service.checkpoint();
        assert_eq!(&result.username, &login_payload.username);
        assert!(result.verify_password(&login_payload.password).is_ok());
    }

    #[tokio::test]
    async fn test_create_duplicated_credential_model() {
        let mut mock = MockAuthDatastore::new();

        mock.expect_get_user_by_username()
            .times(1)
            .returning(|username| {
                Box::pin(future::ready(Ok(Some(UserCredentials {
                    id: Some(ObjectId::new()),
                    username: username.to_string(),
                    ..Faker.fake()
                }))))
            });

        mock.expect_add_user().times(0);

        let login_payload: LoginPayload = Faker.fake();
        let auth_service = MockAuthService::new(mock, MockTokenDatastore::new());
        let result = auth_service.create_credentials(login_payload.clone()).await;

        auth_service.checkpoint();
        assert_eq!(
            result.unwrap_err().to_string(),
            AuthError::Duplicated.to_string()
        );
    }
}
