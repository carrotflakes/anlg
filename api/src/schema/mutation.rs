use async_graphql::*;
use gptcl::model::ChatMessage;
use serde_json::json;

use crate::{clients::gpt::Gpt, repository::Repository};

use super::{
    note::{Message, Role},
    Note,
};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post(&self, ctx: &Context<'_>, content: String) -> Result<Note> {
        let repository = ctx.data::<Repository>().unwrap();
        let gpt = ctx.data::<Gpt>().unwrap();
        let created_at = chrono::Utc::now();
        let mut note = Note {
            id: "".into(),
            content: content.clone(),
            messages: vec![],
            created_at,
            updated_at: created_at,
            deleted_at: None,
        };
        repository.insert_note(&mut note).await?;
        let note = add_companions_comment_to_note(repository, gpt, note).await?;
        repository.update_note(&note).await?;
        Ok(note)
    }

    async fn update_note(&self, ctx: &Context<'_>, input: UpdateNoteInput) -> Result<Note> {
        let repository = ctx.data::<Repository>().unwrap();
        let updated_at = chrono::Utc::now();
        let Some(mut note) = repository.get_note(input.id.to_string()).await? else {
            return Err("not found".into());
        };
        note.content = input.content;
        note.updated_at = updated_at;
        repository.update_note(&note).await?;
        Ok(note)
    }

    async fn delete_note(&self, ctx: &Context<'_>, note_id: ID) -> Result<Note> {
        let repository = ctx.data::<Repository>().unwrap();
        let deleted_at = chrono::Utc::now();
        let Some(mut note) = repository.get_note(note_id.to_string()).await? else {
            return Err("not found".into());
        };
        note.deleted_at = Some(deleted_at);
        repository.update_note(&note).await?;
        Ok(note)
    }

    async fn request_companions_comment(&self, ctx: &Context<'_>, note_id: ID) -> Result<Note> {
        let repository = ctx.data::<Repository>().unwrap();
        let gpt = ctx.data::<Gpt>().unwrap();
        let Some(note) = repository.get_note(note_id.to_string()).await? else {
            return Err("not found".into());
        };
        let note = add_companions_comment_to_note(repository, gpt, note).await?;
        repository.update_note(&note).await?;
        Ok(note)
    }

    async fn add_comment(&self, ctx: &Context<'_>, note_id: ID, content: String) -> Result<Note> {
        let repository = ctx.data::<Repository>().unwrap();
        let gpt = ctx.data::<Gpt>().unwrap();
        let Some(mut note) = repository.get_note(note_id.to_string()).await? else {
            return Err("not found".into());
        };
        note.messages.push(Message {
            role: Role::User,
            content,
            created_at: chrono::Utc::now(),
        });
        repository.update_note(&note).await?;
        let note = add_companions_comment_to_note(repository, gpt, note).await?;
        repository.update_note(&note).await?;
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
    repository: &Repository,
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

    let prompt = format!(
        r#"{{"note":{:?},"comments":{}}}"#,
        note.content,
        serde_json::to_string(
            &note
                .messages
                .iter()
                .map(|m| json!({
                    "role": match m.role {
                        Role::Companion => "you",
                        Role::User => "user",
                    },
                    "text": m.content,
                }))
                .chain([json!({
                    "role": "you",
                    "text": "<TEXT>",
                })])
                .collect::<Vec<_>>()
        )
        .unwrap()
    );
    log::info!("prompt: {:?}", prompt);
    let res = gpt
        .call(&[
            ChatMessage::from_system(
                r#"The user post a note as a JSON. You can leave a comment for the user. Please provide the text to be inserted in place of <TEXT> in the JSON. Your answer must be in the format {{"text":"<TEXT>"}}.
Match the language to the user."#.to_owned(),
            ),
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
