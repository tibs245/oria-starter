use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;

#[cfg(test)]
use chrono::Duration;
#[cfg(test)]
use fake::{Dummy, Faker, Fake, faker::name::raw::Name, locales::EN};
#[cfg(test)]
use fake::faker::internet::en::Password;
#[cfg(test)]
use rand::Rng;
#[cfg(test)]
use rand::seq::SliceRandom;
use thiserror::Error;
use crate::entities::error::AuthError;
use crate::entities::Privileges::{Anonymous, Authenticated, Deny};
use crate::entities::Roles::{Admin, Moderator, SuperAdmin};

pub mod error;

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub username: String,
    pub role: Roles,
}

impl Display for AuthSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}\n; Role : {}\n", self.username, self.clone().role)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Privileges {
    Allow,
    SuperAdminPrivileges,
    AdminPrivileges,
    ModeratorPrivileges,
    Authenticated,
    Anonymous,
    Deny,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Roles {
    SuperAdmin,
    Admin,
    Moderator,
    User,
    None,
}

impl Roles {
    pub fn is_authorized(&self, privileges: Privileges) -> bool {
        match privileges {
            Privileges::Allow => true,
            Privileges::SuperAdminPrivileges => self == &SuperAdmin,
            Privileges::AdminPrivileges => self == &Admin || self.is_authorized(Privileges::SuperAdminPrivileges),
            Privileges::ModeratorPrivileges => self == &Moderator || self.is_authorized(Privileges::AdminPrivileges),
            Authenticated => true,
            Anonymous => false,
            Deny => false,
        }
    }
}


#[cfg(test)]
impl Dummy<Faker> for Roles {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        const ROLES_LIST: &[Roles] = &[Roles::SuperAdmin, Roles::Admin, Roles::Moderator, Roles::User];
        ROLES_LIST.choose(rng).unwrap().clone()
    }
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseRoleError {
    #[error("Cannot parse {0} in Role")]
    NotRole(String)
}

impl FromStr for Roles {
    type Err = ParseRoleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SuperAdmin" => Ok(Roles::SuperAdmin),
            "Admin" => Ok(Roles::Admin),
            "Moderator" => Ok(Roles::Moderator),
            "User" => Ok(Roles::User),
            "None" => Ok(Roles::None),
            _ => Err(Self::Err::NotRole(s.to_string()))
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct UserCredentials {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
    pub roles: Roles,
    pub created_at: DateTime,
    pub last_modified_at: DateTime,
}

impl UserCredentials {
    pub(crate) fn hash_password(password: String) -> String {
        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($pbkdf2-sha256$...)
        Pbkdf2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
    }

    pub fn verify_password(&self, password_hash: &str) -> Result<(), AuthError> {
        let self_password_hash = PasswordHash::new(&self.password).map_err(|_| AuthError::WrongCredentials)?;
        Pbkdf2.verify_password(password_hash.as_bytes(), &self_password_hash).map_err(|_| AuthError::WrongCredentials)
    }
}

#[cfg(test)]
impl Dummy<Faker> for UserCredentials {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, _rng: &mut R) -> Self {
        let now = DateTime::now();
        let password: String = Password(10..500).fake();

        Self {
            id: None,
            username: Name(EN).fake(),
            password: UserCredentials::hash_password(password),
            roles: Faker.fake(),
            created_at: now,
            last_modified_at: now,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Token {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) username: String,
    pub(crate) token_access_identifiers: String,
    pub(crate) token_refresh_identifiers: String,
    pub(crate) created_at: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) revoked_at: Option<DateTime>,
    pub(crate) token_access_expired_at: DateTime,
    pub(crate) token_refresh_expired_at: DateTime,
}

#[cfg(test)]
impl Dummy<Faker> for Token {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, _rng: &mut R) -> Self {
        let now = DateTime::now();

        Token {
            id: None,
            username: Name(EN).fake(),
            token_access_identifiers: (8..500).fake(),
            token_refresh_identifiers: (8..500).fake(),
            created_at: now,
            revoked_at: None,
            token_access_expired_at: DateTime::parse_rfc3339_str((chrono::Utc::now() + Duration::minutes(5)).to_rfc3339()).unwrap(),
            token_refresh_expired_at: DateTime::parse_rfc3339_str((chrono::Utc::now() + Duration::days(1)).to_rfc3339()).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_string = format!("{:?}", self).to_lowercase();
        write!(f, "{}", lowercase_string)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseTokenTypeError {
    #[error("Cannot parse {0} in TokenType")]
    NotTokenType(String)
}

impl FromStr for TokenType {
    type Err = ParseTokenTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "access" => Ok(TokenType::Access),
            "refresh" => Ok(TokenType::Refresh),
            _ => Err(Self::Err::NotTokenType(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;
    use fake::faker::internet::en::Password;
    use super::*;
    use serde_test::{assert_tokens, Token, Configure};
    use once_cell::sync::Lazy;

    #[test]
    fn test_user_credentials_serialization() {
        static NOW: Lazy<DateTime> = Lazy::new(|| { DateTime::now() });
        static NOW_STRING: Lazy<String> = Lazy::new(|| { NOW.timestamp_millis().to_string() });
        static FAKE_USER_CREDENTIALS: Lazy<UserCredentials> = Lazy::new(|| {
            UserCredentials {
                id: None,
                username: Name(EN).fake(),
                password: (8..20).fake::<String>(),
                roles: Roles::SuperAdmin,
                created_at: NOW.to_owned(),
                last_modified_at: NOW.to_owned(),
            }
        });

        assert_tokens(&FAKE_USER_CREDENTIALS.clone().compact(), &[
            Token::Struct { name: "UserCredentials", len: 5 },
            Token::Str("username"),
            Token::Str(&FAKE_USER_CREDENTIALS.username),
            Token::Str("password"),
            Token::Str(&FAKE_USER_CREDENTIALS.password),
            Token::Str("roles"),
            Token::UnitVariant { name: "Roles", variant: "SuperAdmin" },
            Token::Str("created_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&NOW_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("last_modified_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&NOW_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::StructEnd
        ]);
    }

    #[test]
    fn test_user_credentials_with_id_serialization() {
        static OBJECT_ID: Lazy<ObjectId> = Lazy::new(|| { ObjectId::new() });
        static OBJECT_ID_STRING: Lazy<String> = Lazy::new(|| { OBJECT_ID.to_string() });
        static NOW: Lazy<DateTime> = Lazy::new(|| { DateTime::now() });
        static NOW_STRING: Lazy<String> = Lazy::new(|| { NOW.timestamp_millis().to_string() });
        static FAKE_USER_CREDENTIALS: Lazy<UserCredentials> = Lazy::new(|| {
            UserCredentials {
                id: Some(OBJECT_ID.clone()),
                username: Name(EN).fake(),
                password: (8..20).fake::<String>(),
                roles: Roles::SuperAdmin,
                created_at: NOW.to_owned(),
                last_modified_at: NOW.to_owned(),
            }
        });

        assert_tokens(&FAKE_USER_CREDENTIALS.clone().compact(), &[
            Token::Struct { name: "UserCredentials", len: 6 },
            Token::Str("_id"),
            Token::Some,
            Token::Struct { name: "$oid", len: 1 },
            Token::Str("$oid"),
            Token::Str(&OBJECT_ID_STRING),
            Token::StructEnd,
            Token::Str("username"),
            Token::Str(&FAKE_USER_CREDENTIALS.username),
            Token::Str("password"),
            Token::Str(&FAKE_USER_CREDENTIALS.password),
            Token::Str("roles"),
            Token::UnitVariant { name: "Roles", variant: "SuperAdmin" },
            Token::Str("created_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&NOW_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("last_modified_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&NOW_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::StructEnd
        ]);
    }
    #[test]
    fn test_token_identifiers_serialization() {
        static TOKEN: Lazy<crate::entities::Token> = Lazy::new(|| { Faker.fake::<crate::entities::Token>() });
        static CREATED_AT_STRING: Lazy<String> = Lazy::new(|| { TOKEN.created_at.timestamp_millis().to_string() });
        static EXPIRED_ACCESS_AT_STRING: Lazy<String> = Lazy::new(|| { TOKEN.token_access_expired_at.timestamp_millis().to_string() });
        static EXPIRED_REFRESH_AT_STRING: Lazy<String> = Lazy::new(|| { TOKEN.token_refresh_expired_at.timestamp_millis().to_string() });

        assert_tokens(&TOKEN.clone().compact(), &[
            Token::Struct { name: "Token", len: 6 },
            Token::Str("username"),
            Token::Str(&TOKEN.username),
            Token::Str("token_access_identifiers"),
            Token::Str(&TOKEN.token_access_identifiers),
            Token::Str("token_refresh_identifiers"),
            Token::Str(&TOKEN.token_refresh_identifiers),
            Token::Str("created_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&CREATED_AT_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("token_access_expired_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&EXPIRED_ACCESS_AT_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("token_refresh_expired_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&EXPIRED_REFRESH_AT_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::StructEnd,
        ]);
    }


    #[test]
    fn test_token_identifiers_revoked_serialization() {
        static TOKEN: Lazy<crate::entities::Token> = Lazy::new(|| { crate::entities::Token { id: Some(ObjectId::new()), revoked_at: Some(DateTime::now()), ..Faker.fake::<crate::entities::Token>() } });
        static OBJECT_ID_STRING: Lazy<String> = Lazy::new(|| { TOKEN.id.unwrap().to_string() });
        static CREATED_AT_STRING: Lazy<String> = Lazy::new(|| { TOKEN.created_at.timestamp_millis().to_string() });
        static EXPIRED_ACCESS_AT_STRING: Lazy<String> = Lazy::new(|| { TOKEN.token_access_expired_at.timestamp_millis().to_string() });
        static EXPIRED_REFRESH_AT_STRING: Lazy<String> = Lazy::new(|| { TOKEN.token_refresh_expired_at.timestamp_millis().to_string() });
        static REVOKED_AT: Lazy<String> = Lazy::new(|| { TOKEN.revoked_at.unwrap().timestamp_millis().to_string() });

        assert_tokens(&TOKEN.clone().compact(), &[
            Token::Struct { name: "Token", len: 8 },
            Token::Str("_id"),
            Token::Some,
            Token::Struct { name: "$oid", len: 1 },
            Token::Str("$oid"),
            Token::Str(&OBJECT_ID_STRING),
            Token::StructEnd,
            Token::Str("username"),
            Token::Str(&TOKEN.username),
            Token::Str("token_access_identifiers"),
            Token::Str(&TOKEN.token_access_identifiers),
            Token::Str("token_refresh_identifiers"),
            Token::Str(&TOKEN.token_refresh_identifiers),
            Token::Str("created_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&CREATED_AT_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("revoked_at"),
            Token::Some,
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&REVOKED_AT),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("token_access_expired_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&EXPIRED_ACCESS_AT_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("token_refresh_expired_at"),
            Token::Struct { name: "$date", len: 1 },
            Token::Str("$date"),
            Token::Struct { name: "Int64", len: 1 },
            Token::Str("$numberLong"),
            Token::Str(&EXPIRED_REFRESH_AT_STRING),
            Token::StructEnd,
            Token::StructEnd,
            Token::StructEnd,
        ]);
    }

    #[test]
    fn test_user_credential_hash_and_unhash_password() {
        let password: String = Password(10..500).fake();

        let user_credentials: UserCredentials = UserCredentials {
            password: UserCredentials::hash_password(password.clone()),
            ..Faker.fake()
        };

        assert_ne!(user_credentials.password, password);
        assert!(user_credentials.verify_password(&password).is_ok());
        assert!(user_credentials.verify_password(&(password + " ")).is_err());
    }

    #[test]
    fn test_role_from_str() {
        assert_eq!(SuperAdmin.to_string().parse::<Roles>().unwrap(), SuperAdmin);
        assert_eq!(Admin.to_string().parse::<Roles>().unwrap(), Admin);
        assert_eq!(Moderator.to_string().parse::<Roles>().unwrap(), Moderator);
        assert_eq!(Roles::User.to_string().parse::<Roles>().unwrap(), Roles::User);
        assert_eq!(Roles::None.to_string().parse::<Roles>().unwrap(), Roles::None);
        assert_eq!("random".to_string().parse::<Roles>(), Err(ParseRoleError::NotRole("random".to_string())));
    }
}