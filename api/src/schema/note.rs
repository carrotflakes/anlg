use async_graphql::*;
use chrono::DateTime;

#[derive(Clone, SimpleObject)]
pub struct Note {
    pub id: ID,
    pub content: String,
    pub messages: Vec<Message>,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub deleted_at: Option<DateTime<chrono::Utc>>,
}

impl Note {
    pub fn from_json_value(v: serde_json::Value, id: String) -> Self {
        let content = v["content"]["stringValue"].as_str().unwrap().to_owned();
        let messages: Vec<_> = v["messages"]["arrayValue"]["values"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|v| Message::from_json_value(v["entityValue"]["properties"].clone()))
                    .collect()
            })
            .unwrap_or_else(|| vec![]);
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
            messages,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::json!({
            "content": {
                "excludeFromIndexes": true,
                "stringValue": self.content
            },
            "messages": {
                "arrayValue": {
                    "values": self.messages.iter().map(|m|
                        serde_json::json!({
                            "entityValue": {
                                "properties": m.to_json_value()
                            }
                        })
                    ).collect::<Vec<_>>()
                }
            },
            "createdAt": {
                "timestampValue": self.created_at
            },
            "updatedAt": {
                "timestampValue": self.updated_at
            },
            "deletedAt": if let Some(deleted_at) = self.deleted_at {
                serde_json::json!({"timestampValue": deleted_at})
            } else {
                serde_json::json!({"nullValue": null})
            }
        })
    }
}

#[derive(Clone, SimpleObject)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
}

impl Message {
    pub fn from_json_value(v: serde_json::Value) -> Self {
        let role = match v["role"]["stringValue"].as_str().unwrap() {
            "user" => Role::User,
            "companion" => Role::Companion,
            _ => unreachable!(),
        };
        let content = v["content"]["stringValue"].as_str().unwrap().to_owned();
        let created_at =
            DateTime::parse_from_rfc3339(v["createdAt"]["timestampValue"].as_str().unwrap())
                .unwrap()
                .into();
        Self {
            role,
            content,
            created_at,
        }
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::json!({
            "role": {
                "stringValue": match self.role {
                    Role::User => "user",
                    Role::Companion => "companion",
                }
            },
            "content": {
                "stringValue": self.content
            },
            "createdAt": {
                "timestampValue": self.created_at
            }
        })
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq)]
pub enum Role {
    User,
    Companion,
}
