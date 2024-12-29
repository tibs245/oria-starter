    use mongodb::bson::oid::ObjectId;
    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;
    use crate::datastore::AuthDatastoreError;
    use crate::entities::UserCredentials;

    
    #[derive(Clone)]
    pub struct AuthMemoryDriver {
    }

    static USER_LIST: Lazy<Mutex<Vec<UserCredentials>>> = Lazy::new(|| Mutex::new(Vec::new()));
    impl AuthMemoryDriver {
        
        pub async fn add_user(&self, user: UserCredentials) -> Result<UserCredentials, AuthDatastoreError> {
            let new_user_credentials = UserCredentials {
                id: Some(ObjectId::new()),
                ..user
            };

            USER_LIST.lock().await.push(new_user_credentials.clone());
            
            Ok(new_user_credentials)
        }

         pub async fn get_user_by_username(&self, username: &str) -> Option<UserCredentials> {
            if let Some(user_credentials) = USER_LIST.lock().await.iter().find(|user_credentials: &&UserCredentials| {
                user_credentials.username == username
            }) {
                return Some(user_credentials.clone())
            }

            return None;
        }
    }