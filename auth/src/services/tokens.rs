use mongodb::bson::DateTime;
use crate::datastore::{AuthDatastore, TokenDatastore};
use crate::entities::error::AuthError;
use crate::entities::{Token, UserCredentials};
use crate::services::{AuthTokensService, AuthService};
use crate::utils::auth_claims::AuthClaims;
use crate::utils::validate_token::{IntoClaims, TokenString};
use crate::views::payload::RefreshTokenPayload;
use crate::views::response::AuthBody;

impl<AuthDatastoreImpl, TokenDatastoreImpl> AuthTokensService for AuthService<AuthDatastoreImpl, TokenDatastoreImpl>
    where AuthDatastoreImpl: AuthDatastore, TokenDatastoreImpl: TokenDatastore
{
    fn parse_auth_claims_from_refresh_payload(refresh_token_payload: RefreshTokenPayload) -> Result<AuthClaims, AuthError> {
        let untrusted_token = TokenString(refresh_token_payload.refresh_token);
        let claims = untrusted_token.try_into_claims()?;

        AuthClaims::try_from(&claims).map_err(|_| AuthError::InvalidToken)
    }


    async fn validate_token(&self, auth_claims: &AuthClaims) -> Result<Token, AuthError> {
        let token_to_validate = self.token_datastore.get_token(&auth_claims.token_identifier).await.map_err(|_| AuthError::InvalidToken)?;

        if token_to_validate.is_none() {
            return Err(AuthError::InvalidToken)
        }

        let token_state = token_to_validate.unwrap();

        if token_state.revoked_at.is_some() || token_state.token_refresh_expired_at > DateTime::now() {
            return Err(AuthError::InvalidToken);
        }

        Ok(token_state)
    }

    async fn try_get_user_token(&self, token: &Token) -> Result<UserCredentials, AuthError> {
        let user = self.auth_datastore.get_user_by_username(&token.username)
            .await
            .map_err(|_| AuthError::ServerError)?;

        if user.is_none() {
            return Err(AuthError::InvalidToken)
        }

        Ok(user.unwrap())
    }

    async fn generate_token(&self, user: &UserCredentials) -> Result<AuthBody, AuthError> {
        let (access_token, refresh_token, tokens) = Token::generate_tokens(user).await.map_err(|_| AuthError::ServerError)?;
        self.token_datastore.add_tokens(tokens).await.map_err(|_| AuthError::ServerError)?;

        Ok(AuthBody {
            token: access_token,
            refresh_token
        })
    }

    async fn refresh_tokens(&self, refresh_token_payload: RefreshTokenPayload) -> Result<AuthBody, AuthError> {
        let auth_claims = Self::parse_auth_claims_from_refresh_payload(refresh_token_payload)?;

        let token_state = self.validate_token(&auth_claims).await?;

        let user = self.try_get_user_token(&token_state).await?;

        // Revoke actual token and generate new token from actual token
        self.token_datastore.revoke_token(&auth_claims.token_identifier).await.map_err(|_| AuthError::ServerError)?;

        self.generate_token(&user).await
    }
}

