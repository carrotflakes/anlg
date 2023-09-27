use async_graphql::*;

use crate::repository::Repository;

use super::Note;

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
}
