use std::sync::Arc;
use axum::{Extension, Json};
use crate::entities::error::AuthError;
use crate::services::{AuthTokensService, AuthValidCredentialsService};
use crate::views::payload::{LoginPayload};
use crate::views::response::AuthBody;

pub async fn login<AuthServiceImpl: AuthTokensService + AuthValidCredentialsService>(auth_service: Extension<Arc<AuthServiceImpl>>, Json(payload): Json<LoginPayload>) -> Result<Json<AuthBody>, AuthError> {
    let user = auth_service.is_valid_credentials(payload.username, payload.password).await?;
    let tokens = auth_service.generate_token(&user).await?;

    Ok(Json(tokens))
}

#[cfg(test)]
mod test {
    use std::future;
    use std::sync::Arc;
    use axum::{Extension, Json};
    use fake::{Fake, Faker};
    use fake::faker::internet::en::{Password, Username};
    use mockall::predicate::eq;
    use mongodb::bson::oid::ObjectId;
    use once_cell::sync::Lazy;
    use crate::controller::login::login;
    use crate::datastore::{MockAuthDatastore, MockTokenDatastore};
    use crate::entities::{Token, UserCredentials};
    use crate::entities::error::AuthError;
    use crate::services::{MockAuthService};
    use crate::utils::settings::AuthSettings;
    use crate::views::payload::LoginPayload;

    #[tokio::test]
    async fn test_unit_login() {
        AuthSettings::init_fake();

        let mut mock_auth_datastore = MockAuthDatastore::new();
        let mut mock_tokens_datastore = MockTokenDatastore::new();
        let username: String = Username().fake();
        static PASSWORD: Lazy<String> = Lazy::new(|| Password(10..500).fake());

        mock_auth_datastore.expect_get_user_by_username().with(eq(username.clone())).times(1).returning(|username| {
            return Box::pin(future::ready(Ok(Some(UserCredentials {
                username: username.to_string(),
                password: UserCredentials::hash_password(PASSWORD.clone()),
                ..Faker.fake::<UserCredentials>()
            }))));
        });
        
        mock_tokens_datastore.expect_add_tokens().times(1).returning(|token| {
            Box::pin(future::ready(Ok(Token {
                id: Some(ObjectId::new()),
                ..token
            })))
        });


        let extension = MockAuthService::new(mock_auth_datastore, mock_tokens_datastore);

        assert!(login(Extension(Arc::new(extension)), Json(LoginPayload {
            username: username.clone(),
            password: PASSWORD.clone()
        })).await.is_ok());
    }


    #[tokio::test]
    async fn test_unit_bad_login() {
        AuthSettings::init_fake();

        let mut mock_auth_datastore = MockAuthDatastore::new();
        let mut mock_tokens_datastore = MockTokenDatastore::new();
        let username: String = Username().fake();
        let password: String = Password(10..500).fake();

        mock_auth_datastore.expect_get_user_by_username().with(eq(username.clone())).times(1).returning(|_username| {
            return Box::pin(future::ready(Ok(None)));
        });

        mock_tokens_datastore.expect_add_tokens().times(0);

        let extension = MockAuthService::new(mock_auth_datastore, mock_tokens_datastore);

        assert_eq!(login(Extension(Arc::new(extension)), Json(LoginPayload {
            username: username.clone(),
            password: password.clone()
        })).await.unwrap_err().to_string(), AuthError::WrongCredentials.to_string());
    }
}