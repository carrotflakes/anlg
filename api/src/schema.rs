use async_graphql::*;
use chrono::DateTime;
use serde_json::json;

use crate::gcdatastore::Client;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn notes(&self, ctx: &Context<'_>) -> Vec<Note> {
        let client = ctx.data::<Client>().unwrap();
        let notes = client
            .run_query(&json!({
                "query": {
                    "limit": 50,
                    "kind": [{
                        "name": "note"
                    }]
                }
            }))
            .await;
        notes
            .into_iter()
            .map(|(path, v)| Note {
                id: path.id,
                content: v["content"].as_object().unwrap()["stringValue"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                created_at: DateTime::parse_from_rfc3339(
                    v["createdAt"].as_object().unwrap()["timestampValue"]
                        .as_str()
                        .unwrap(),
                )
                .unwrap()
                .into(),
            })
            .collect()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post(&self, ctx: &Context<'_>, content: String) -> Result<bool> {
        let client = ctx.data::<Client>().unwrap();
        let properties = json!({
            "content": {
                "excludeFromIndexes": true,
                "stringValue": content
            },
            "createdAt": {
                "excludeFromIndexes": true,
                "timestampValue": chrono::Utc::now()
            }
        });
        client
            .commit(crate::gcdatastore::Commit::Insert {
                kind: "note".to_string(),
                properties,
            })
            .await;
        Ok(true)
    }

    async fn delete(&self, ctx: &Context<'_>, note_id: String) -> Result<bool> {
        let client = ctx.data::<Client>().unwrap();
        client
            .commit(crate::gcdatastore::Commit::Delete {
                kind: "note".to_string(),
                id: note_id,
            })
            .await;
        Ok(true)
    }
}

#[derive(Clone, SimpleObject)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
}
