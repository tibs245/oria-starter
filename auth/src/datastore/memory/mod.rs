

#[cfg(test)]
pub mod memory_driver;

#[cfg(test)]
mod test {
    use crate::datastore::{AuthDatastore, AuthDatastoreError};
    use crate::datastore::memory::memory_driver::AuthMemoryDriver;
    use crate::entities::UserCredentials;

    #[derive(Clone)]
    pub struct AuthDatastoreMemory {
        auth_memory_driver: AuthMemoryDriver
    }

    /// Use memory to emulate datastore
    /// It's designed for integration test usage only
    impl AuthDatastore for AuthDatastoreMemory {
        async fn add_user(&self, user: UserCredentials) -> Result<UserCredentials, AuthDatastoreError> {
            if user.id.is_some() {
                return Err(AuthDatastoreError::BadFormat("Id is already defined".to_string()))
            }

            self.auth_memory_driver.add_user(user).await
        }

        async fn get_user_by_username(&self, username: &str) -> Result<Option<UserCredentials>, AuthDatastoreError> {
            Ok(self.auth_memory_driver.get_user_by_username(username).await)
        }
    }
}
