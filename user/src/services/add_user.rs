use crate::datastore::UserDatastore;
use crate::entities::error::UserError;
use crate::services::{UserAddService, UserService};
use crate::views::user_payload::{UserWithCredentialsPayload};
use auth_module::entities::UserCredentials;
use auth_module::services::{AuthCreateCredentialsService};
use std::error::Error;

impl<AuthServiceImpl, UserDatastoreImpl> UserAddService
for UserService<AuthServiceImpl, UserDatastoreImpl>
where
    AuthServiceImpl: AuthCreateCredentialsService + 'static + Send + Sync,
    UserDatastoreImpl: UserDatastore + 'static + Send + Sync,
{
    async fn add_user(
        &self,
        user_with_credential_payload: UserWithCredentialsPayload,
    ) -> Result<UserCredentials, Box<dyn Error + Send + Sync + 'static>> {
        if self
            .user_datastore
            .get_user_by_username(&user_with_credential_payload.username)
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

#[cfg(test)]
mod tests {
    use std::{error::Error, future::Future, pin::Pin};
    use super::*;
    use fake::{Fake, Faker};
    use mongodb::bson::oid::ObjectId;
    use std::future;
    use mockall::mock;
    use auth_module::services::{AuthCreateCredentialsService};
    use auth_module::views::payload::LoginPayload;
    use crate::datastore::MockUserDatastore;
    use crate::entities::user::User;

    mock! {
    pub AuthCreateCredentialsService {}

    impl AuthCreateCredentialsService for AuthCreateCredentialsService {
        fn create_credentials(
            &self,
            auth_payload: LoginPayload
        ) -> impl std::future::Future<Output = Result<auth_module::entities::UserCredentials, Box<(dyn Error + Send + Sync + 'static)>>>;
    }
        
    impl Clone for AuthCreateCredentialsService {
        fn clone(&self) -> Self {
            self
        }
    }
}

    #[tokio::test]
    async fn test_add_user_model() {
        let mut mock_user_datastore = MockUserDatastore::new();
        let mut mock_create_credentials_service = MockAuthCreateCredentialsService::new();

        mock_user_datastore.expect_get_user_by_username()
            .times(1)
            .returning(|_username| Box::pin(future::ready(Ok(None))));


        mock_create_credentials_service.expect_create_credentials()
            .times(1)
            .returning(|login_payload: LoginPayload| {
                Box::pin(future::ready(Ok(UserCredentials {
                    id: Some(ObjectId::new()),
                    ..login_payload.into()
                })))
            });

        mock_user_datastore.expect_add_user()
            .times(1)
            .returning(|user: User| {
                Box::pin(future::ready(Ok(User {
                    id: Some(ObjectId::new()),
                    ..user
                })))
            });

        let user_payload: UserWithCredentialsPayload = Faker.fake();
        let user_service = UserService::new(mock_create_credentials_service, mock_user_datastore);
        let result = user_service
            .add_user(user_payload.clone())
            .await
            .expect("Unable create user with credential with mock");

        //user_service.checkpoint();
        assert_eq!(&result.username, &user_payload.username);
        assert!(result.verify_password(&user_payload.password).is_ok());
    }
}
