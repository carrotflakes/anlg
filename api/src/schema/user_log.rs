use async_graphql::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct UserLog {
    // #[serde(rename = "_firestore_id")]
    // #[graphql(skip)]
    // pub id: Option<ID>,
    pub r#type: String,
    pub message: String,
    pub datetime: DateTime<chrono::Utc>,
}

#[ComplexObject]
impl UserLog {
    // pub async fn id(&self) -> &ID {
    //     self.id.as_ref().unwrap()
    // }
}
