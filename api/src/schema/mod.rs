mod chat;
mod mutation;
mod note;
mod query;

pub use chat::*;
pub use mutation::Mutation;
pub use note::*;
pub use query::Query;


mod item {
    use async_graphql::*;

    #[derive(Interface)]
    #[graphql(
        field(name = "id", ty = "&ID"),
        field(name = "created_at", ty = "&chrono::DateTime<chrono::Utc>"),
    )]
    pub enum Item {
        Note(super::note::Note),
    }
}
