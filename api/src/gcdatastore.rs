use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::gcp::TokenGetter;

pub struct Client {
    reqwest: reqwest::Client,
    project_id: String,
    token_getter: TokenGetter,
}

impl Client {
    pub fn new(credentials_json_path: &str) -> Self {
        Client {
            reqwest: reqwest::Client::new(),
            project_id: "optical-loop-227914".to_owned(),
            token_getter: TokenGetter::from_credentials_json(credentials_json_path),
        }
    }

    pub async fn run_query<T: Serialize>(&self, query: &T) -> Vec<Value> {
        let access_token = self.token_getter.get().await;
        let client = &self.reqwest;
        let res = client
            .post(&format!(
                "https://datastore.googleapis.com/v1/projects/{}:runQuery?key=AIzaSyArs-ffex8SEmPhrJw2xzXkKidNstR2p9c",
                self.project_id
            ))
            .bearer_auth(&access_token)
            .json(&query)
            .send()
            .await
            .unwrap();
        let res = res.json::<QueryResult>().await.unwrap();
        res.batch
            .entity_results
            .into_iter()
            .map(|er| er.entity.properties)
            .collect()
    }

    pub async fn commit(&self, content: &str) {
        let client = &self.reqwest;
        let access_token = self.token_getter.get().await;
        let res = client
            .post(&format!(
                "https://datastore.googleapis.com/v1/projects/{}:commit",
                self.project_id
            ))
            .bearer_auth(&access_token)
            .json(&json!({
                "mode": "NON_TRANSACTIONAL",
                "mutations": [
                    {
                        "insert": {
                            "key": {
                                "partitionId": {
                                    "namespaceId": ""
                                },
                                "path": [
                                    {
                                        "kind": "note"
                                    }
                                ]
                            },
                            "properties": {
                                "content": {
                                    "excludeFromIndexes": true,
                                    "stringValue": content
                                },
                                "createdAt": {
                                    "excludeFromIndexes": true,
                                    "timestampValue": chrono::Utc::now()
                                }
                            }
                        }
                    }
                ]
            }))
            .send()
            .await
            .unwrap();
        dbg!(res.json::<Value>().await.unwrap());
    }
}

#[derive(Deserialize)]
struct QueryResult {
    batch: Batch,
}

#[derive(Deserialize)]
struct Batch {
    #[serde(rename = "entityResultType")]
    entity_result_type: String,
    #[serde(rename = "entityResults")]
    entity_results: Vec<EntityResult>,
}

#[derive(Deserialize)]
struct EntityResult {
    entity: Entity,
}

#[derive(Deserialize)]
struct Entity {
    key: Key,
    properties: Value,
}

#[derive(Deserialize)]
struct Key {
    #[serde(rename = "partitionId")]
    partition_id: Value,
    path: Vec<Value>,
}
