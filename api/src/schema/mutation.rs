use async_graphql::*;
use serde_json::json;

use crate::gcdatastore::Client;

use super::Note;

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

    async fn simple_gpt_request(&self, ctx: &Context<'_>, prompt: String) -> Result<String> {
        let gpt = ctx.data::<crate::gpt::Gpt>().unwrap();
        let res = gpt.simple_request(&prompt).await;
        Ok(res)
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
