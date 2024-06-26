use async_graphql::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Note {
    #[serde(rename = "_firestore_id")]
    #[graphql(skip)]
    pub id: Option<ID>,
    pub content: String,
    pub messages: Vec<Message>,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    #[serde(default)]
    #[serde(with = "firestore::serialize_as_null_timestamp")]
    pub deleted_at: Option<DateTime<chrono::Utc>>,
}

#[ComplexObject]
impl Note {
    pub async fn id(&self) -> &ID {
        self.id.as_ref().unwrap()
    }
}

#[derive(Clone, SimpleObject, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum Role {
    User,
    Companion,
}
