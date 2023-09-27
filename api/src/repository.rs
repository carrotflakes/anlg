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
        let mut req = json!({
            "query": {
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
        });
        if !include_deleted {
            req["query"]["filter"] = json!({
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
        if let Some(limit) = limit {
            req["query"]["limit"] = json!(limit);
        }

        let notes = self.datastore.run_query(&req).await?;
        let notes = notes
            .into_iter()
            .map(|(path, v)| Note::from_json_value(v, path.id))
            .collect();
        Ok(notes)
    }

    pub async fn get_note(&self, id: String) -> Result<Option<Note>> {
        let notes = self
            .datastore
            .run_query(&get_note_query(id.to_string()))
            .await?;
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

pub fn get_note_query(note_id: String) -> serde_json::Value {
    json!({
        "query": {
            "limit": 1,
            "kind": [{
                "name": "note"
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
                                    "kind": "note",
                                    "id": note_id
                                }
                            ]
                        }
                    }
                }
            }
        }
    })
}
