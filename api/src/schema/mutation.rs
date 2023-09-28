use async_graphql::*;
use gptcl::model::ChatMessage;

use crate::{clients::gpt::Gpt, repository::Repository, service::add_companions_comment_to_note};

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
