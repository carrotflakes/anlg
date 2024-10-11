use async_graphql::*;
use chrono::DateTime;

use crate::repository::Repository;

use super::{Chat, Note, UserLog};

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn notes(&self, ctx: &Context<'_>, include_deleted: bool) -> Result<Vec<Note>> {
        let repository = ctx.data::<Repository>().unwrap();
        repository
            .get_notes(include_deleted, None)
            .await
            .map_err(Error::from)
    }

    async fn note(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Note>> {
        let repository = ctx.data::<Repository>().unwrap();
        repository
            .get_note(id.to_string())
            .await
            .map_err(Error::from)
    }

    async fn chats(
        &self,
        ctx: &Context<'_>,
        include_deleted: bool,
        limit: Option<usize>,
    ) -> Result<Vec<Chat>> {
        let repository = ctx.data::<Repository>().unwrap();
        repository
            .get_chats(include_deleted, limit)
            .await
            .map_err(Error::from)
    }

    async fn chat(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Chat>> {
        let repository = ctx.data::<Repository>().unwrap();
        repository
            .get_chat(id.to_string())
            .await
            .map_err(Error::from)
    }

    async fn user_logs(
        &self,
        ctx: &Context<'_>,
        before: Option<DateTime<chrono::Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<UserLog>> {
        let repository = ctx.data::<Repository>().unwrap();
        repository
            .get_user_logs(before, limit.unwrap_or(100))
            .await
            .map_err(Error::from)
    }
}
