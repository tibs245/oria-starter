use mongodb::{Collection, Database};
use mongodb::bson::{Bson, doc};
use crate::datastore::{AuthDatastore, AuthDatastoreError};
use crate::entities::UserCredentials;

/// This DataStore is the main datastore use for this module
///
/// This use mongodb driver to communicate with collection of user
#[derive(Clone)]
pub struct MongoAuthDatastore {
    collection: Collection<UserCredentials>
}


impl MongoAuthDatastore {
    const DEFAULT_COLLECTION_NAME: &'static str = "auth";

    pub fn new(database: &Database) -> Self {
        Self {
            collection: database.collection::<UserCredentials>(Self::DEFAULT_COLLECTION_NAME)
        }
    }
}

impl AuthDatastore for MongoAuthDatastore {
    async fn add_user(&self, user: UserCredentials) -> Result<UserCredentials, AuthDatastoreError> {
        let transaction_inserted = self.collection.insert_one(&user).await;

        if let Bson::ObjectId(inserted_id) = transaction_inserted.unwrap().inserted_id {
            Ok(UserCredentials {
                id: Some(inserted_id),
                ..user
            })
        } else {
            Err(AuthDatastoreError::ProvidersError)
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<UserCredentials>, AuthDatastoreError> {
        self.collection.find_one(doc! { "username": username }).await.map_err(|_| AuthDatastoreError::ProvidersError)
    }
}
 