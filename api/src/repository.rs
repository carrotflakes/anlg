use firestore::{path, FirestoreDb, FirestoreQueryDirection};

use crate::schema::{Chat, Note};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Repository {
    db: FirestoreDb,
}

impl Repository {
    pub fn new(db: FirestoreDb) -> Self {
        Self { db }
    }

    pub async fn get_notes(
        &self,
        include_deleted: bool,
        limit: Option<usize>,
    ) -> Result<Vec<Note>> {
        let mut q = self.db.fluent().select().from("notes");
        if !include_deleted {
            q = q.filter(|q| q.field(path!(Note::deleted_at)).is_null());
        }
        if let Some(limit) = limit {
            q = q.limit(limit as u32);
        }
        let notes: Vec<Note> = q
            .order_by([(path!(Note::created_at), FirestoreQueryDirection::Descending)])
            .obj()
            .query()
            .await?;
        Ok(notes)
    }

    pub async fn get_note(&self, id: String) -> Result<Option<Note>> {
        Ok(self
            .db
            .fluent()
            .select()
            .by_id_in("notes")
            .obj()
            .one(&id)
            .await?)
    }

    pub async fn insert_note(&self, note: &mut Note) -> Result<()> {
        let res: Note = self
            .db
            .fluent()
            .insert()
            .into("notes")
            .generate_document_id()
            .object(note)
            .execute()
            .await?;
        note.id = res.id;
        Ok(())
    }

    pub async fn update_note(&self, note: &Note) -> Result<()> {
        self.db
            .fluent()
            .update()
            .in_col("notes")
            .document_id(&note.id.clone().unwrap().to_string())
            .object(note)
            .execute()
            .await?;
        Ok(())
    }

    pub async fn get_chats(
        &self,
        include_deleted: bool,
        limit: Option<usize>,
    ) -> Result<Vec<Chat>> {
        let mut q = self.db.fluent().select().from("chats");
        if !include_deleted {
            q = q.filter(|q| q.field(path!(Chat::deleted_at)).is_null());
        }
        if let Some(limit) = limit {
            q = q.limit(limit as u32);
        }
        Ok(
            q.order_by([(path!(Chat::created_at), FirestoreQueryDirection::Descending)])
                .obj()
                .query()
                .await?,
        )
    }

    pub async fn get_chat(&self, id: String) -> Result<Option<Chat>> {
        Ok(self
            .db
            .fluent()
            .select()
            .by_id_in("chats")
            .obj()
            .one(&id)
            .await?)
    }

    pub async fn insert_chat(&self, chat: &mut Chat) -> Result<()> {
        let res: Chat = self
            .db
            .fluent()
            .insert()
            .into("chats")
            .generate_document_id()
            .object(chat)
            .execute()
            .await?;
        chat.id = res.id;
        Ok(())
    }

    pub async fn update_chat(&self, chat: &Chat) -> Result<()> {
        self.db
            .fluent()
            .update()
            .in_col("chats")
            .document_id(&chat.id.clone().unwrap().to_string())
            .object(chat)
            .execute()
            .await?;
        Ok(())
    }
}
