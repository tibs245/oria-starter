#[cfg(test)]
use fake::Dummy;
use mongodb::bson::DateTime;
use serde::Deserialize;
use crate::entities::{Roles, UserCredentials};

#[cfg(test)]
use serde::Serialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Serialize, Clone))]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

impl From<LoginPayload> for UserCredentials {
    fn from(login_payload: LoginPayload) -> Self {
        let now = DateTime::now();

        Self {
            id: None,
            username: login_payload.username,
            roles: Roles::User,
            password: UserCredentials::hash_password(login_payload.password),
            created_at: now,
            last_modified_at: now,
        }
    }
}


#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Serialize, Clone, Dummy))]
pub struct RefreshTokenPayload {
    pub(crate) refresh_token: String
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Dummy, Fake, Faker};
    use fake::faker::internet::fr_fr::{Password, Username};
    use once_cell::sync::Lazy;
    use rand::Rng;
    use serde_test::{assert_tokens, Token};

    impl Dummy<Faker> for LoginPayload {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _rng: &mut R) -> Self {
            Self {
                username: Username().fake(),
                password: Password(10..500).fake()
            }
        }
    }

    #[test]
    fn test_payload_from_user_credential_login() {
        let user_payload = Faker.fake::<LoginPayload>();
        let new_user_credentials = UserCredentials::from(user_payload.clone());

        assert!(new_user_credentials.id.is_none());
        assert_eq!(new_user_credentials.username, user_payload.username);
        assert_ne!(new_user_credentials.password, user_payload.password);
        assert!(new_user_credentials.verify_password(&user_payload.password).is_ok());
        assert!(new_user_credentials.verify_password(&(user_payload.password + " ")).is_err());
        assert_eq!(new_user_credentials.created_at, new_user_credentials.last_modified_at);
    }
    
    #[test]
    fn test_serialization_login_payload() {
        static USER_PAYLOAD: Lazy<LoginPayload> = Lazy::new(|| Faker.fake::<LoginPayload>());
        
        assert_tokens(&USER_PAYLOAD.clone(), &[
            Token::Struct { name: "LoginPayload", len: 2 },
            Token::Str("username"),
            Token::Str(&USER_PAYLOAD.username),
            Token::Str("password"),
            Token::Str(&USER_PAYLOAD.password),
            Token::StructEnd
        ]);
    }


    #[test]
    fn test_serialization_refresh_token_payload() {
        static REFRESH_TOKEN_PAYLOAD: Lazy<RefreshTokenPayload> = Lazy::new(|| Faker.fake::<RefreshTokenPayload>());

        assert_tokens(&REFRESH_TOKEN_PAYLOAD.clone(), &[
            Token::Struct { name: "RefreshTokenPayload", len: 1 },
            Token::Str("refresh_token"),
            Token::Str(&REFRESH_TOKEN_PAYLOAD.refresh_token),
            Token::StructEnd
        ]);
    }
}