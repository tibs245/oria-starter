use mongodb::{Collection, Database};
use mongodb::bson::{Bson, doc};
use crate::datastore::{UserDatastore, UserDatastoreError};
use crate::entities::user::User;

#[derive(Clone)]
pub struct MongoUserDatastore {
    collection: Collection<User>
}

impl MongoUserDatastore {
    const COLLECTION_NAME: &'static str = "users";

    pub fn new(database: &Database) -> Self {
        Self {
            collection: database.collection::<User>(Self::COLLECTION_NAME)
        }
    }
}

impl UserDatastore for MongoUserDatastore {
    async fn add_user(&self, user: User) -> Result<User, UserDatastoreError> {
        let transaction_inserted = self.collection.insert_one(&user).await;

        if let Bson::ObjectId(inserted_id) = transaction_inserted.unwrap().inserted_id {
            Ok(User {
                id: Some(inserted_id),
                ..user
            })
        } else {
            Err(UserDatastoreError::ProvidersError)
        }
    }

    /// Returns an `Result<Option<User>, UserDatastoreError>` representing a user with the given username, or `None` if no such user exists.
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, UserDatastoreError> {
        self.collection.find_one(doc! { "username": username }).await.map_err(|_| UserDatastoreError::ProvidersError)
    }
}