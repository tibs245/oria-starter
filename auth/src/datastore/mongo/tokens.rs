
use mongodb::{Collection, Database};
use mongodb::bson::{Bson, doc, DateTime};
use crate::datastore::{TokenDatastoreError, TokenDatastore};
use crate::entities::{Token};
use futures::stream::TryStreamExt;

/// This DataStore is the main datastore use for this module
///
/// This use mongodb driver to communicate with collection of user
#[derive(Clone)]
pub struct MongoTokenDatastore {
    collection: Collection<Token>
}


impl MongoTokenDatastore {
    const DEFAULT_COLLECTION_NAME: &'static str = "tokens";

    pub fn new(database: &Database) -> Self {
        return Self {
            collection: database.collection::<Token>(Self::DEFAULT_COLLECTION_NAME)
        };
    }
}

impl TokenDatastore for MongoTokenDatastore {
    async fn add_tokens(&self, token: Token) -> Result<Token, TokenDatastoreError> {
        let token_inserted = self.collection.insert_one(&token).await;

        if let Bson::ObjectId(inserted_id) = token_inserted.unwrap().inserted_id {
            Ok(Token {
                id: Some(inserted_id),
                ..token
            })
        } else {
            Err(TokenDatastoreError::ProvidersError)
        }
    }
    async fn get_token(&self, token_identifier: &str) -> Result<Option<Token>, TokenDatastoreError> {
        self.collection.find_one(doc! { "token_refresh_identifiers": token_identifier }).await.map_err(|_| TokenDatastoreError::ProvidersError)
    }

    async fn get_tokens_for_user(&self, username: &str) -> Result<Vec<Token>, TokenDatastoreError> {
        self.collection.find(doc! { "username": username }).await.map_err(|_| TokenDatastoreError::ProvidersError)?.try_collect().await.map_err(|_| TokenDatastoreError::InternalError)
    }

    async fn revoke_token(&self, token_identifier: &str) -> Result<(), TokenDatastoreError> {
        println!("Try update of : {}", &token_identifier);
        let result = self.collection.update_one(doc! { "token_refresh_identifiers": token_identifier },  doc! { "$set": doc! { "revoked_at": DateTime::now() }}).await.map_err(|_| TokenDatastoreError::ProvidersError)?;

        println!("Modified count : {:?}", result.modified_count);
        if result.modified_count == 1 {
            Ok(())
        } else {
            Err(TokenDatastoreError::InternalError)
        }
    }
}
