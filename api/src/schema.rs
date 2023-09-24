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

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post(&self, ctx: &Context<'_>, content: String) -> Result<Note> {
        let client = ctx.data::<Client>().unwrap();
        let created_at = chrono::Utc::now();
        let properties = json!({
            "content": {
                "excludeFromIndexes": true,
                "stringValue": content
            },
            "createdAt": {
                "timestampValue": created_at
            },
            "updatedAt": {
                "timestampValue": created_at
            }
        });
        let res = client
            .commit(crate::gcdatastore::Commit::Insert {
                kind: "note".to_string(),
                properties,
            })
            .await;
        Ok(Note {
            id: res.mutation_results[0].key.as_ref().unwrap().path[0]
                .id
                .clone()
                .into(),
            content,
            created_at,
            updated_at: created_at,
            deleted_at: None,
        })
    }

    async fn update_note(&self, ctx: &Context<'_>, input: UpdateNoteInput) -> Result<Note> {
        let client = ctx.data::<Client>().unwrap();
        let updated_at = chrono::Utc::now();
        let notes = client
            .run_query(&get_note_query(input.id.to_string()))
            .await;
        let Some(note) = notes.get(0).cloned() else {
            return Err("not found".into());
        };
        let properties = json!({
            "content": {
                "excludeFromIndexes": true,
                "stringValue": input.content
            },
            "createdAt": {
                "timestampValue": note.1["createdAt"]["timestampValue"].clone()
            },
            "updatedAt": {
                "timestampValue": updated_at
            },
            "deletedAt": null
        });
        let res = client
            .commit(crate::gcdatastore::Commit::Update {
                kind: "note".to_string(),
                id: input.id.to_string(),
                properties,
            })
            .await;
        Ok(Note {
            id: res.mutation_results[0].key.as_ref().unwrap().path[0]
                .id
                .clone()
                .into(),
            content: input.content,
            created_at: note.1["createdAt"]["timestampValue"]
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),
            updated_at,
            deleted_at: None,
        })
    }

    async fn delete_note(&self, ctx: &Context<'_>, note_id: String) -> Result<Note> {
        let client = ctx.data::<Client>().unwrap();
        let deleted_at = chrono::Utc::now();
        let notes = client.run_query(&get_note_query(note_id.clone())).await;
        let Some(note) = notes.get(0).cloned() else {
            return Err("not found".into());
        };
        let properties = json!({
            "content": {
                "excludeFromIndexes": true,
                "stringValue": note.1["content"]["stringValue"].clone()
            },
            "createdAt": {
                "timestampValue": note.1["createdAt"]["timestampValue"].clone()
            },
            "updatedAt": {
                "timestampValue": note.1["updatedAt"]["timestampValue"].clone()
            },
            "deletedAt": {
                "timestampValue": deleted_at
            }
        });
        let _ = client
            .commit(crate::gcdatastore::Commit::Update {
                kind: "note".to_string(),
                id: note_id.clone(),
                properties,
            })
            .await;
        Ok(Note {
            id: note_id.into(),
            content: note.1["content"]["stringValue"]
                .as_str()
                .unwrap()
                .to_owned(),
            created_at: note.1["createdAt"]["timestampValue"]
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),
            updated_at: note.1["updatedAt"]["timestampValue"]
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),
            deleted_at: Some(deleted_at),
        })
    }
}

#[derive(Clone, SimpleObject)]
pub struct Note {
    pub id: ID,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub deleted_at: Option<DateTime<chrono::Utc>>,
}

impl Note {
    pub fn from_json_value(v: serde_json::Value, id: String) -> Self {
        let content = v["content"]["stringValue"].as_str().unwrap().to_owned();
        let created_at =
            DateTime::parse_from_rfc3339(v["createdAt"]["timestampValue"].as_str().unwrap())
                .unwrap()
                .into();
        let updated_at = DateTime::parse_from_rfc3339(
            v["updatedAt"]["timestampValue"]
                .as_str()
                .or(v["createdAt"]["timestampValue"].as_str())
                .unwrap(),
        )
        .unwrap()
        .into();
        let deleted_at = v["deletedAt"]["timestampValue"]
            .as_str()
            .map(|s| DateTime::parse_from_rfc3339(s).unwrap().into());
        Self {
            id: id.into(),
            content,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}

#[derive(InputObject)]
pub struct UpdateNoteInput {
    pub id: ID,
    pub content: String,
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
