use async_graphql::*;
use serde_json::json;

use crate::gcdatastore::Client;

use super::{
    note::{Message, Role},
    Note,
};

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
            messages: vec![],
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
        let mut note = Note::from_json_value(note.1, input.id.to_string());
        note.content = input.content;
        note.updated_at = updated_at;
        let properties = note.to_json_value();
        let _ = client
            .commit(crate::gcdatastore::Commit::Update {
                kind: "note".to_string(),
                id: input.id.to_string(),
                properties,
            })
            .await;
        Ok(note)
    }

    async fn delete_note(&self, ctx: &Context<'_>, note_id: ID) -> Result<Note> {
        let client = ctx.data::<Client>().unwrap();
        let deleted_at = chrono::Utc::now();
        let notes = client.run_query(&get_note_query(note_id.to_string())).await;
        let Some(note) = notes.get(0).cloned() else {
            return Err("not found".into());
        };
        let mut note = Note::from_json_value(note.1, note_id.to_string());
        note.deleted_at = Some(deleted_at);
        let properties = note.to_json_value();
        let _ = client
            .commit(crate::gcdatastore::Commit::Update {
                kind: "note".to_string(),
                id: note_id.to_string(),
                properties,
            })
            .await;
        Ok(note)
    }

    async fn request_companions_comment(&self, ctx: &Context<'_>, note_id: ID) -> Result<Note> {
        let client = ctx.data::<Client>().unwrap();
        let notes = client.run_query(&get_note_query(note_id.to_string())).await;
        let Some(note) = notes.get(0).cloned() else {
            return Err("not found".into());
        };
        let mut note = Note::from_json_value(note.1, note_id.to_string());
        let prompt = if note.messages.is_empty() {
            format!("You're a companion. The user posts a note here: \n\n# User note\n{:?}\n\nPlease write a comment shortly for the user in about 10 words.", note.content)
        } else {
            format!(
            "You're a companion. The user posts a note and comments under the note is here: \n\n# User note\n{:?}\n\n# Comments {}\n\nPlease write a comment for the user in about 10 words.",
            note.content,
            note.messages
                .iter()
                .map(|m| format!("\n{:?}: {:?}", m.role, m.content))
                .collect::<String>()
            )
        };
        println!("prompt: {:?}", prompt);
        let gpt = ctx.data::<crate::gpt::Gpt>().unwrap();
        let res = gpt.simple_request(&prompt).await;
        note.messages.push(Message {
            role: Role::Companion,
            content: res,
            created_at: chrono::Utc::now(),
        });
        let properties = note.to_json_value();
        let _ = client
            .commit(crate::gcdatastore::Commit::Update {
                kind: "note".to_string(),
                id: note_id.to_string(),
                properties,
            })
            .await;
        Ok(note)
    }

    async fn add_comment(&self, ctx: &Context<'_>, note_id: ID, content: String) -> Result<Note> {
        let client = ctx.data::<Client>().unwrap();
        let notes = client.run_query(&get_note_query(note_id.to_string())).await;
        let Some(note) = notes.get(0).cloned() else {
            return Err("not found".into());
        };
        let mut note = Note::from_json_value(note.1, note_id.to_string());
        note.messages.push(Message {
            role: Role::User,
            content,
            created_at: chrono::Utc::now(),
        });
        let properties = note.to_json_value();
        let _ = client
            .commit(crate::gcdatastore::Commit::Update {
                kind: "note".to_string(),
                id: note_id.to_string(),
                properties,
            })
            .await;
        Ok(note)
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
