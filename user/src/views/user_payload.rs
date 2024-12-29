use mongodb::bson::DateTime;
use serde::{Deserialize};
use crate::entities::user::User;
#[cfg(test)]
use fake::Dummy;
#[cfg(test)]
use fake::faker::internet::en::{ Username, FreeEmail, Password };
use auth_module::views::payload::LoginPayload;

#[cfg_attr(test, derive(Dummy))]
#[derive(Debug, Clone, Deserialize)]
pub struct UserPayload {
    #[cfg_attr(test, dummy(faker = "Username()"))]
    pub username: String,
    #[cfg_attr(test, dummy(faker = "FreeEmail()"))]
    pub email: String,
}

impl From<UserPayload> for User {
    fn from(user_payload: UserPayload) -> Self {
        Self {
            id: None,
            username: user_payload.username,
            email: user_payload.email,
            created_at: DateTime::now(),
        }
    }
}

#[cfg_attr(test, derive(Dummy))]
#[derive(Debug, Clone, Deserialize)]
pub struct UserWithCredentialsPayload {
    #[cfg_attr(test, dummy(faker = "Username()"))]
    pub username: String,
    #[cfg_attr(test, dummy(faker = "FreeEmail()"))]
    pub email: String,
    #[cfg_attr(test, dummy(faker = "Password(10..300)"))]
    pub password: String,
}
impl From<UserWithCredentialsPayload> for LoginPayload {
    fn from(user_payload: UserWithCredentialsPayload) -> Self {
        Self {
            username: user_payload.username,
            password: user_payload.password
        }
    }
}


impl From<UserWithCredentialsPayload> for UserPayload {
    fn from(user_payload: UserWithCredentialsPayload) -> Self {
        Self {
            username: user_payload.username,
            email: user_payload.email
        }
    }
}

impl From<UserWithCredentialsPayload> for User {
    fn from(user: UserWithCredentialsPayload) -> Self {
        UserPayload::from(user).into()
    }
}


#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use auth_module::views::payload::LoginPayload;
    use crate::entities::user::User;
    use crate::views::user_payload::{UserPayload, UserWithCredentialsPayload};

    #[test]
    fn test_user_from_user_payload () {
        let user_payload: UserPayload = Faker.fake();
        let user: User = user_payload.clone().into();

        assert!(user.id.is_none());
        assert_eq!(user.email, user_payload.email);
        assert_eq!(user.username, user_payload.username);
    }


    #[test]
    fn test_user_payload_from_user_with_credentials_payload () {
        let user_payload: UserWithCredentialsPayload = Faker.fake();
        let user: UserPayload = user_payload.clone().into();

        assert_eq!(user.email, user_payload.email);
        assert_eq!(user.username, user_payload.username);
    }


    #[test]
    fn test_login_payload_from_user_with_credential_payload () {
        let user_payload: UserWithCredentialsPayload = Faker.fake();
        let user: LoginPayload = user_payload.clone().into();

        assert_eq!(user.username, user_payload.username);
        assert_eq!(user.password, user_payload.password);
    }
}