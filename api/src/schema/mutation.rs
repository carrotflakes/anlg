use async_graphql::*;
use gptcl::model::ChatMessage;
use serde_json::json;

use crate::{
    clients::{
        gcdatastore::{Client as DSClient, Commit},
        gpt::Gpt,
    },
    schema::query::get_note_query,
};

use super::{
    note::{Message, Role},
    Note,
};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post(&self, ctx: &Context<'_>, content: String) -> Result<Note> {
        let datastore = ctx.data::<DSClient>().unwrap();
        let gpt = ctx.data::<Gpt>().unwrap();
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
        let res = datastore
            .commit(Commit::Insert {
                kind: "note".to_string(),
                properties: properties.clone(),
            })
            .await;
        let note_id = res.mutation_results[0].key.as_ref().unwrap().path[0]
            .id
            .clone();
        let note = Note::from_json_value(properties, note_id.clone());
        let note = add_companions_comment_to_note(datastore, gpt, note).await?;
        let properties = note.to_json_value();
        let _ = datastore
            .commit(Commit::Update {
                kind: "note".to_string(),
                id: note_id.to_string(),
                properties,
            })
            .await;
        Ok(note)
    }

    async fn update_note(&self, ctx: &Context<'_>, input: UpdateNoteInput) -> Result<Note> {
        let datastore = ctx.data::<DSClient>().unwrap();
        let updated_at = chrono::Utc::now();
        let notes = datastore
            .run_query(&get_note_query(input.id.to_string()))
            .await;
        let Some(note) = notes.get(0).cloned() else {
            return Err("not found".into());
        };
        let mut note = Note::from_json_value(note.1, input.id.to_string());
        note.content = input.content;
        note.updated_at = updated_at;
        let properties = note.to_json_value();
        let _ = datastore
            .commit(Commit::Update {
                kind: "note".to_string(),
                id: input.id.to_string(),
                properties,
            })
            .await;
        Ok(note)
    }

    async fn delete_note(&self, ctx: &Context<'_>, note_id: ID) -> Result<Note> {
        let datastore = ctx.data::<DSClient>().unwrap();
        let deleted_at = chrono::Utc::now();
        let notes = datastore
            .run_query(&get_note_query(note_id.to_string()))
            .await;
        let Some(note) = notes.get(0).cloned() else {
            return Err("not found".into());
        };
        let mut note = Note::from_json_value(note.1, note_id.to_string());
        note.deleted_at = Some(deleted_at);
        let properties = note.to_json_value();
        let _ = datastore
            .commit(Commit::Update {
                kind: "note".to_string(),
                id: note_id.to_string(),
                properties,
            })
            .await;
        Ok(note)
    }

    async fn request_companions_comment(&self, ctx: &Context<'_>, note_id: ID) -> Result<Note> {
        let datastore = ctx.data::<DSClient>().unwrap();
        let gpt = ctx.data::<Gpt>().unwrap();
        let notes = datastore
            .run_query(&get_note_query(note_id.to_string()))
            .await;
        let Some(note) = notes.get(0).cloned() else {
            return Err("not found".into());
        };
        let mut note = Note::from_json_value(note.1, note_id.to_string());
        let note = add_companions_comment_to_note(datastore, gpt, note).await?;
        let properties = note.to_json_value();
        let _ = datastore
            .commit(Commit::Update {
                kind: "note".to_string(),
                id: note_id.to_string(),
                properties,
            })
            .await;
        Ok(note)
    }

    async fn add_comment(&self, ctx: &Context<'_>, note_id: ID, content: String) -> Result<Note> {
        let datastore = ctx.data::<DSClient>().unwrap();
        let gpt = ctx.data::<Gpt>().unwrap();
        let notes = datastore
            .run_query(&get_note_query(note_id.to_string()))
            .await;
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
        let _ = datastore
            .commit(Commit::Update {
                kind: "note".to_string(),
                id: note_id.to_string(),
                properties,
            })
            .await;
        let note = add_companions_comment_to_note(datastore, gpt, note).await?;
        let properties = note.to_json_value();
        let _ = datastore
            .commit(Commit::Update {
                kind: "note".to_string(),
                id: note_id.to_string(),
                properties,
            })
            .await;
        Ok(note)
    }

    async fn simple_gpt_request(&self, ctx: &Context<'_>, prompt: String) -> Result<String> {
        let gpt = ctx.data::<Gpt>().unwrap();
        let res = gpt.call(&[ChatMessage::from_user(prompt)]).await?;
        Ok(res.content.unwrap())
    }
}

#[derive(InputObject)]
pub struct UpdateNoteInput {
    pub id: ID,
    pub content: String,
}

pub async fn add_companions_comment_to_note(
    datastore: &DSClient,
    gpt: &Gpt,
    mut note: Note,
) -> Result<Note> {
    //     let prompt = if note.messages.is_empty() {
    //         format!(
    //             r"You're a companion. The user posts a note here:
    // # User note
    // {:?}

    // Please write a comment for the user in about 10 words. No quotes needed. Don't expect the user to respond.",
    //             note.content
    //         )
    //     } else {
    //         format!(
    //             r"You're a companion. The user posts a note and comments under the note is here:

    // # User note
    // {:?}

    // # Comments (in chronological order){}

    // Please write a comment for the user in about 10 words. No quotes needed.",
    //             note.content,
    //             note.messages
    //                 .iter()
    //                 .map(|m| format!(r#"\n{{"role":"{:?}",text:{:?}}}"#, m.role, m.content))
    //                 .collect::<String>()
    //         )
    //     };

    let note_info = json!({
        "note": note.content,
        "messages": note.messages.iter().map(|m|
            json!({
                "role": match m.role {
                    Role::Companion => "you",
                    Role::User => "user",
                },
                "text": m.content,
            })
        ).chain([json!({
            "role": "you",
            "text": "<TEXT>",
        })]).collect::<Vec<_>>(),
    });
    let prompt = format!(
        r#"The user has posted a note. You can leave a comment for the user. Please provide the text to be inserted in place of <TEXT> in the following JSON. Your answer must be in the format {{"text":"<TEXT>"}}.

{}"#,
        serde_json::to_string(&note_info).unwrap()
    );
    log::info!("prompt: {:?}", prompt);
    let res = gpt
        .call(&[
            // ChatMessage::from_system(
            //     "you must respond as if you were treating a friend.".to_owned(),
            // ),
            ChatMessage::from_user(prompt),
        ])
        .await?;
    let content_json = res.content.unwrap();
    let content = serde_json::from_str::<serde_json::Value>(&content_json)
        .map_err(|e| Error::from(e))
        .and_then(|v| {
            v.get("text")
                .ok_or(Error::new("Unexpected response"))
                .and_then(|v| {
                    v.as_str()
                        .map(|s| s.to_owned())
                        .ok_or(Error::new("Unexpected response"))
                })
        })
        .map_err(|e| Error::from(format!("Invalid json: {:?}: {:?}", content_json, e)))?;
    log::info!("GPT response: {:?}", content);
    note.messages.push(Message {
        role: Role::Companion,
        content,
        created_at: chrono::Utc::now(),
    });
    Ok(note)
}
