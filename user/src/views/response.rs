use mongodb::bson::DateTime;
use serde::{Serialize};
use auth_module::entities::{Roles, UserCredentials};
use crate::entities::user::User;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Clone, PartialEq))]
pub struct UserPrivateDetails {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) roles: Roles,
    #[serde(with = "mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub(crate) created_at: DateTime,
    #[serde(with = "mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub(crate) last_modified_at: DateTime,
}

impl UserPrivateDetails {
    pub fn from_credential_and_user_profile(user_credential: UserCredentials, user_profile: User) -> Self {
        Self {
            username: user_profile.username,
            email: user_profile.email,
            roles: user_credential.roles,
            created_at: user_credential.created_at,
            last_modified_at: user_credential.last_modified_at,
        }
    }
}