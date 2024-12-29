use std::error::Error;
use std::ops::Add;
use chrono::{DateTime, Duration, TimeDelta, Utc};
use mongodb::bson;
use pasetors::claims::{Claims};
use pasetors::public;
use crate::entities::error::AuthError;
use crate::entities::{Token, TokenType, UserCredentials};

use crate::utils::settings::AuthSettings;

impl Token {
    const ACCESS_TOKEN_LIFETIME: TimeDelta = Duration::minutes(10);
    const REFRESH_TOKEN_LIFETIME: TimeDelta = Duration::days(1);

    fn generate_token_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
    fn generate_access_token(user: &UserCredentials) -> Result<(String, DateTime<Utc>, String), AuthError> {
        let token_id = Self::generate_token_id();
        let expiration = Utc::now().add(Self::ACCESS_TOKEN_LIFETIME);
        let mut claims = Claims::new().map_err(|_| AuthError::TokenCreation)?;
        claims.token_identifier(&token_id.clone()).expect("Unable to insert token id");
        claims.subject(&TokenType::Access.to_string()).map_err(|_| AuthError::TokenCreation)?;
        claims.expiration(&expiration.to_rfc3339()).expect("Cannot define expiration");
        claims.add_additional("username", user.username.to_string()).map_err(|_| AuthError::TokenCreation)?;
        claims.add_additional("role", user.roles.to_string()).map_err(|_| AuthError::TokenCreation)?;

        // Send the authorized token
        Ok((token_id, expiration, public::sign(&AuthSettings::get_secret_key(), &claims, None, Some(b"implicit assertion")).map_err(|_| AuthError::TokenCreation)?))
    }
    fn generate_refresh_token(user: &UserCredentials) -> Result<(String, DateTime<Utc>, String), AuthError> {
        let token_id = Self::generate_token_id();
        let expiration = Utc::now().add(Self::REFRESH_TOKEN_LIFETIME);
        let mut claims = Claims::new().map_err(|_| AuthError::TokenCreation)?;
        claims.subject(&TokenType::Refresh.to_string()).map_err(|_| AuthError::TokenCreation)?;
        claims.expiration(&expiration.to_rfc3339()).expect("Cannot define expiration");
        claims.token_identifier(&token_id.clone()).expect("Unable to insert token id");
        claims.add_additional("username", user.username.to_string()).map_err(|_| AuthError::TokenCreation)?;

        // Send the authorized token
        Ok((token_id, expiration, public::sign(&AuthSettings::get_secret_key(), &claims, None, Some(b"implicit assertion")).map_err(|_| AuthError::TokenCreation)?))
    }

    pub async fn generate_tokens(user: &UserCredentials) -> Result<(String, String, Self), Box<dyn Error>> {
        let (access_token_id, access_expired_at, access_token) = Self::generate_access_token(&user).map_err(|error| Box::new(error))?;
        let (refresh_token_id, refresh_expired_at, refresh_token) = Self::generate_refresh_token(&user).map_err(|error| Box::new(error))?;

        Ok((access_token, refresh_token, Self {
            id: None,
            username: user.username.to_string(),
            token_access_identifiers: access_token_id,
            token_refresh_identifiers: refresh_token_id,
            token_access_expired_at: bson::DateTime::parse_rfc3339_str(access_expired_at.to_rfc3339()).unwrap(),
            token_refresh_expired_at: bson::DateTime::parse_rfc3339_str(refresh_expired_at.to_rfc3339()).unwrap(),
            created_at: bson::DateTime::now(),
            revoked_at: None
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{Fake, Faker};
    use pasetors::claims::ClaimsValidationRules;
    use pasetors::Public;
    use pasetors::token::UntrustedToken;
    use pasetors::version4::V4;

    fn validate_access_token(token_id: String, token: String, user_credential: UserCredentials) {
        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token = UntrustedToken::<Public, V4>::try_from(&token).expect("Unable parse string to token");
        let trusted_token = public::verify(&AuthSettings::get_public_key(), &untrusted_token, &validation_rules, None, Some(b"implicit assertion")).expect("Unable to verify token with this public key");


        let mut validation_rules = ClaimsValidationRules::new();
        validation_rules.validate_subject_with(&TokenType::Access.to_string());

        assert_eq!(trusted_token.payload_claims().unwrap().get_claim("jti").unwrap().to_string().trim_matches('"'), token_id);
        assert!(validation_rules.validate_claims(&trusted_token.payload_claims().unwrap()).is_ok());
        assert_eq!(trusted_token.payload_claims().unwrap().get_claim("username").unwrap().to_string().trim_matches('"'), user_credential.username);
        assert_eq!(trusted_token.payload_claims().unwrap().get_claim("role").unwrap().to_string().trim_matches('"'), user_credential.roles.to_string());
    }


    fn validate_refresh_token(token_id: String, token: String, user_credential: UserCredentials) {
        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token = UntrustedToken::<Public, V4>::try_from(&token).expect("Unable parse string to token");
        let trusted_token = public::verify(&AuthSettings::get_public_key(), &untrusted_token, &validation_rules, None, Some(b"implicit assertion")).expect("Unable to verify token with this public key");


        let mut validation_rules = ClaimsValidationRules::new();
        validation_rules.validate_subject_with(&TokenType::Refresh.to_string());

        assert_eq!(trusted_token.payload_claims().unwrap().get_claim("jti").unwrap().to_string().trim_matches('"'), token_id);
        assert!(validation_rules.validate_claims(&trusted_token.payload_claims().unwrap()).is_ok());
        assert_eq!(trusted_token.payload_claims().unwrap().get_claim("username").unwrap().to_string().trim_matches('"'), user_credential.username);

    }

    #[test]
    fn test_generate_access_token() {
        AuthSettings::init_fake();
        let user_credential = Faker.fake();


        let (token_id, expiration, token_generated) = Token::generate_access_token(&user_credential).expect("Unable to generate access token");
        assert!(Utc::now() < expiration);
        validate_access_token(token_id, token_generated, user_credential);
   }

    #[test]
    fn test_generate_refresh_token() {
        AuthSettings::init_fake();
        let user_credential = Faker.fake();


        let (token_id, expiration, token_generated) = Token::generate_refresh_token(&user_credential).expect("Unable to generate access token");
        assert!(Utc::now() < expiration);
        validate_refresh_token(token_id, token_generated, user_credential);
   }
}