use async_graphql::*;
use chrono::DateTime;

#[derive(Clone, SimpleObject)]
pub struct Note {
    pub id: ID,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub deleted_at: Option<DateTime<chrono::Utc>>,
}

impl Note {
    pub fn from_json_value(v: serde_json::Value, id: String) -> Self {
        let content = v["content"]["stringValue"].as_str().unwrap().to_owned();
        let created_at =
            DateTime::parse_from_rfc3339(v["createdAt"]["timestampValue"].as_str().unwrap())
                .unwrap()
                .into();
        let updated_at = DateTime::parse_from_rfc3339(
            v["updatedAt"]["timestampValue"]
                .as_str()
                .or(v["createdAt"]["timestampValue"].as_str())
                .unwrap(),
        )
        .unwrap()
        .into();
        let deleted_at = v["deletedAt"]["timestampValue"]
            .as_str()
            .map(|s| DateTime::parse_from_rfc3339(s).unwrap().into());
        Self {
            id: id.into(),
            content,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
