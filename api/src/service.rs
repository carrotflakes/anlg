use async_graphql::*;
use gptcl::model::ChatMessage;
use serde_json::json;

use crate::{
    clients::gpt::{new_request, Gpt},
    repository::Repository,
    schema::{self, Chat, Message, Note, Role},
};

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

    let context = if true {
        let mut recent_logs = repository.get_notes(false, Some(10)).await?;
        recent_logs.retain(|n| n.id != note.id);

        format!(
            r#"{{"recentOtherNotes":{}}}"#,
            serde_json::to_string(
                &recent_logs
                    .iter()
                    .map(|n| json!({
                        "note": n.content,
                        "createdAt": n.created_at.to_rfc3339(),
                    }))
                    .collect::<Vec<_>>()
            )
            .unwrap()
        )
    } else {
        "null".to_owned()
    };

    let prompt = format!(
        r#"{{"note":{:?},"createdAt":{:?},"comments":{},"context":{}}}"#,
        note.content,
        note.created_at.to_rfc3339(),
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
        .unwrap(),
        context,
    );
    log::info!("prompt: {:?}", prompt);
    let res = gpt
        .call(&new_request(vec![
            ChatMessage::from_system(
                r#"The user post a note. You can leave a comment for the user. Please provide the text to be inserted in place of <TEXT> in the JSON.
Your answer must be in the format {"text":"<TEXT>"}.
Match the language to the user.
You may also consider context information."#.to_owned(),
            ),
            ChatMessage::from_user(prompt),
        ]))
        .await?;
    let content_json = res.choices[0].message.content.as_ref().unwrap();
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

pub async fn add_companions_comment_to_chat(gpt: &Gpt, mut chat: Chat) -> Result<Chat> {
    let res = gpt
        .call(&new_request(vec![ChatMessage::from_user(
            chat.messages.last().unwrap().content.clone(),
        )]))
        .await?;
    let content_json = res.choices[0].message.content.as_ref().unwrap();
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
    chat.messages.push(schema::ChatMessage {
        role: Role::Companion,
        content,
        created_at: chrono::Utc::now(),
    });
    Ok(chat)
}
