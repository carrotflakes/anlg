use async_graphql::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use super::Role;

#[derive(Clone, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Chat {
    #[serde(rename = "_firestore_id")]
    #[graphql(skip)]
    pub id: Option<ID>,
    pub messages: Vec<ChatMessage>,
    pub created_at: DateTime<chrono::Utc>,
    #[serde(default)]
    #[serde(with = "firestore::serialize_as_null_timestamp")]
    pub deleted_at: Option<DateTime<chrono::Utc>>,
}

#[ComplexObject]
impl Chat {
    pub async fn id(&self) -> &ID {
        self.id.as_ref().unwrap()
    }
}

#[derive(Clone, SimpleObject, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
}
