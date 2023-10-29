use std::sync::Arc;

use serde_json::json;

use crate::{
    clients::gcdatastore::{Client as DSClient, Commit},
    schema::Note,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Repository {
    datastore: Arc<DSClient>,
}

impl Repository {
    pub fn new(datastore: Arc<DSClient>) -> Self {
        Self { datastore }
    }

    pub async fn get_notes(
        &self,
        include_deleted: bool,
        limit: Option<usize>,
    ) -> Result<Vec<Note>> {
        let query = query_list("note", Some(("createdAt", true)), limit, include_deleted);
        let notes = self.datastore.run_query(&query).await?;
        let notes = notes
            .into_iter()
            .map(|(path, v)| Note::from_json_value(v, path.id))
            .collect();
        Ok(notes)
    }

    pub async fn get_note(&self, id: String) -> Result<Option<Note>> {
        let notes = self.datastore.run_query(&query_by_id("note", &id)).await?;
        let note = notes
            .into_iter()
            .map(|(path, v)| Note::from_json_value(v, path.id))
            .next();
        Ok(note)
    }

    pub async fn insert_note(&self, note: &mut Note) -> Result<()> {
        let properties = note.to_json_value();
        let res = self
            .datastore
            .commit(Commit::Insert {
                kind: "note".to_string(),
                properties,
            })
            .await?;
        let note_id = res.mutation_results[0].key.as_ref().unwrap().path[0]
            .id
            .clone();
        note.id = async_graphql::ID::from(&note_id);
        let properties = note.to_json_value();
        let _ = self
            .datastore
            .commit(Commit::Update {
                kind: "note".to_string(),
                id: note_id.clone(),
                properties,
            })
            .await;
        Ok(())
    }

    pub async fn update_note(&self, note: &Note) -> Result<()> {
        let properties = note.to_json_value();
        let _ = self
            .datastore
            .commit(Commit::Update {
                kind: "note".to_string(),
                id: note.id.to_string(),
                properties,
            })
            .await;
        Ok(())
    }
}

pub fn query_by_id(kind: &str, id: &str) -> serde_json::Value {
    json!({
        "query": {
            "limit": 1,
            "kind": [{
                "name": kind
            }],
            "filter": {
                "propertyFilter": {
                    "property": {
                        "name": "__key__"
                    },
                    "op": "EQUAL",
                    "value": {
                        "keyValue": {
                            "partitionId": {
                                "namespaceId": ""
                            },
                            "path": [
                                {
                                    "kind": kind,
                                    "id": id
                                }
                            ]
                        }
                    }
                }
            }
        }
    })
}

pub fn query_list(
    kind: &str,
    order: Option<(&str, bool)>,
    limit: Option<usize>,
    include_deleted: bool,
) -> serde_json::Value {
    let mut query = json!({
        "query": {
            "kind": [
                {
                    "name": kind
                }
            ]
        }
    });
    if let Some((order_by, desc)) = order {
        query["query"]["order"] = json!([
            {
                "property": {
                    "name": order_by
                },
                "direction": if desc { "DESCENDING" } else { "ASCENDING" }
            }
        ]);
    }
    if let Some(limit) = limit {
        query["query"]["limit"] = json!(limit);
    }
    if !include_deleted {
        query["query"]["filter"] = json!({
            "propertyFilter": {
                "property": {
                    "name": "deletedAt"
                },
                "op": "EQUAL",
                "value": {
                    "nullValue": "NULL_VALUE"
                }
            }
        });
    }
    query
}
