use serde::Serialize;
use crate::entities::{Roles, UserCredentials};

#[cfg(test)]
use serde::Deserialize;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize, Clone, PartialEq))]
pub struct CredentialsPrivateDetails {
    pub(crate) username: String,
    pub(crate) roles: Roles,
    pub(crate) created_at: String,
    pub(crate) last_modified_at: String,
}

impl From<UserCredentials> for CredentialsPrivateDetails {
    fn from(user_credentials: UserCredentials) -> Self {
        Self {
            username: user_credentials.username,
            roles: user_credentials.roles,
            created_at: user_credentials.created_at.try_to_rfc3339_string().unwrap(),
            last_modified_at: user_credentials.last_modified_at.try_to_rfc3339_string().unwrap(),
        }
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize, PartialEq))]
pub struct AuthBody {
    pub(crate) token: String,
    pub(crate) refresh_token: String,
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use once_cell::sync::Lazy;
    use super::*; // bring the AuthBody struct into scope
    use serde_test::{assert_tokens, Configure, Token}; // import the necessary functions from serde_test

    #[test]
    fn test_auth_body_serialization() {
        let auth_body = AuthBody {
            token: "sample_token".to_string(),
            refresh_token: "sample_refresh_token".to_string(),
        };

        assert_tokens(
            &auth_body,
            &[
                Token::Struct { name: "AuthBody", len: 2 },
                Token::Str("token"),
                Token::Str("sample_token"),
                Token::Str("refresh_token"),
                Token::Str("sample_refresh_token"),
                Token::StructEnd,
            ],
        );
    }


    #[test]
    fn test_user_private_details_serialization() {
        static USER_CREDENTIALS: Lazy<UserCredentials> = Lazy::new(|| Faker.fake());
        static USER_PRIVATE_DETAILS: Lazy<CredentialsPrivateDetails> = Lazy::new(|| CredentialsPrivateDetails::from(USER_CREDENTIALS.clone()));
        static USER_PRIVATE_DETAILS_ROLE_STRING: Lazy<String> = Lazy::new(|| USER_PRIVATE_DETAILS.roles.to_string());
        static USER_PRIVATE_DETAILS_CREATED_AT_STRING: Lazy<String> = Lazy::new(|| USER_PRIVATE_DETAILS.created_at.to_string());
        static USER_PRIVATE_DETAILS_LAST_MODIFIED_AT_STRING: Lazy<String> = Lazy::new(|| USER_PRIVATE_DETAILS.last_modified_at.to_string());
        assert_tokens(
            &USER_PRIVATE_DETAILS.clone().readable(),
            &[
                Token::Struct { name: "UserPrivateDetails", len: 4 },
                Token::Str("username"),
                Token::Str(&USER_PRIVATE_DETAILS.username),
                Token::Str("roles"),
                Token::UnitVariant { name: "Roles", variant: &USER_PRIVATE_DETAILS_ROLE_STRING },
                Token::Str("created_at"),
                Token::Str(&USER_PRIVATE_DETAILS_CREATED_AT_STRING),
                Token::Str("last_modified_at"),
                Token::Str(&USER_PRIVATE_DETAILS_LAST_MODIFIED_AT_STRING),
                Token::StructEnd,
            ],
        );
    }
}