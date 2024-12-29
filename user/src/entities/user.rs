use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    #[serde(with = "mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
}
