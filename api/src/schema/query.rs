use async_graphql::*;
use serde_json::json;

use crate::clients::gcdatastore::Client as DSClient;

use super::Note;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn notes(&self, ctx: &Context<'_>) -> Vec<Note> {
        let datastore = ctx.data::<DSClient>().unwrap();
        let notes = datastore
            .run_query(&json!({
                "query": {
                    "limit": 50,
                    "kind": [{
                        "name": "note"
                    }],
                    "order": [
                        {
                            "property": {
                                "name": "createdAt"
                            },
                            "direction": "DESCENDING"
                        }
                    ]
                }
            }))
            .await;
        notes
            .into_iter()
            .map(|(path, v)| Note::from_json_value(v, path.id))
            .collect()
    }
}
