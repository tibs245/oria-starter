use crate::datastore::{AuthDatastore, TokenDatastore};
use crate::entities::error::AuthError;
use crate::entities::UserCredentials;
use crate::services::{AuthService, AuthValidCredentialsService};

impl<AuthDatastoreImpl, TokenDatastoreImpl> AuthValidCredentialsService for AuthService<AuthDatastoreImpl, TokenDatastoreImpl>
    where AuthDatastoreImpl: AuthDatastore, TokenDatastoreImpl: TokenDatastore
{
    async fn is_valid_credentials(&self, username: String, password: String) -> Result<UserCredentials, AuthError> {
        // Check if the user sent the credentials
        if username.is_empty() || password.is_empty() {
            return Err(AuthError::MissingCredentials);
        }

        let user_credentials_option = self.auth_datastore.get_user_by_username(&username).await.map_err(|_| AuthError::ServerError)?;

        user_credentials_option
            .filter(|user_credentials| user_credentials.verify_password(&password).is_ok())
            .ok_or(AuthError::WrongCredentials)
    }
}


#[cfg(test)]
mod test {
    use std::future;
    use fake::{Fake, Faker};
    use fake::faker::internet::en::{Password, Username};
    use mongodb::bson::oid::ObjectId;
    use once_cell::sync::Lazy;
    use crate::datastore::{MockAuthDatastore, MockTokenDatastore};
    use crate::entities::error::AuthError;
    use crate::entities::UserCredentials;
    use crate::services::{AuthValidCredentialsService, MockAuthService};

    #[tokio::test]
    pub async fn test_is_valid_auth_without_user_serverside() {
        let mut mock_auth_datastore = MockAuthDatastore::new();
        let username: String = Username().fake();
        let password: String = Password(10..500).fake();

        mock_auth_datastore.expect_get_user_by_username()
            .times(1)
            .returning(|_username|  {
                Box::pin(future::ready(
                    Ok(None)
                ))
            });

        let auth_service = MockAuthService::new(mock_auth_datastore, MockTokenDatastore::new());
        let result = auth_service.is_valid_credentials(username, password).await;

        auth_service.checkpoint();

        assert_eq!(result.unwrap_err().to_string(), AuthError::WrongCredentials.to_string());
    }

    #[tokio::test]
    pub async fn test_is_not_valid_credentials_auth() {
        let mut mock_auth_datastore = MockAuthDatastore::new();
        let username: String = Username().fake();
        let password: String = Password(10..500).fake();

        mock_auth_datastore.expect_get_user_by_username()
            .times(1)
            .returning(|username|  {
                Box::pin(future::ready(
                    Ok(Some(
                        UserCredentials {
                            id: Some(ObjectId::new()),
                            username: username.to_string(),
                            ..Faker.fake()
                        }
                    ))
                ))
            });

        let auth_service = MockAuthService::new(mock_auth_datastore, MockTokenDatastore::new());
        let result = auth_service.is_valid_credentials(username, password).await;

        auth_service.checkpoint();
        
        assert_eq!(result.unwrap_err().to_string(), AuthError::WrongCredentials.to_string());
    }


    #[tokio::test]
    pub async fn test_is_valid_credentials() {
        let mut mock_auth_datastore = MockAuthDatastore::new();
        let username: String = Username().fake();
        static PASSWORD: Lazy<String> = Lazy::new(|| Password(10..500).fake());

        mock_auth_datastore.expect_get_user_by_username()
            .times(1)
            .returning(|username|  {
                Box::pin(future::ready(
                    Ok(Some(
                        UserCredentials {
                            id: Some(ObjectId::new()),
                            username: username.to_string(),
                            password: UserCredentials::hash_password(PASSWORD.to_string()),
                            ..Faker.fake()
                        }
                    ))
                ))
            });

        let auth_service = MockAuthService::new(mock_auth_datastore, MockTokenDatastore::new());
        let result = auth_service.is_valid_credentials(username, PASSWORD.clone()).await;

        auth_service.checkpoint();

        assert!(result.is_ok());
    }
}
