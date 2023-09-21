use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct Client {
    reqwest: reqwest::Client,
    project_id: String,
    access_token: String,
}

impl Client {
    pub fn new() -> Self {
        Client {
            reqwest: reqwest::Client::new(),
            project_id: "optical-loop-227914".to_owned(),
            access_token: std::env::var("GCP_ACCESS_TOKEN").unwrap(),
        }
    }

    pub async fn run_query<T: Serialize>(&self, query: &T) -> Vec<Value> {
        // let client = awc::Client::new();
        // let client = &self.awc_client;
        // let res = client.post(format!(
        //             "https://datastore.googleapis.com/v1/projects/{}:runQuery",
        //             self.project_id
        //         ))
        //     .insert_header(("Authorization", format!("Bearer {}", self.access_token)))
        //     .insert_header(("Content-Type", "application/json"))
        //     .send_json(&query)
        //     .await;
        // res.unwrap().json().await.unwrap()
        let client = &self.reqwest;
        let res = client
            .post(&format!(
                "https://datastore.googleapis.com/v1/projects/{}:runQuery",
                self.project_id
            ))
            .bearer_auth(&self.access_token)
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
        let res = client
            .post(&format!(
                "https://datastore.googleapis.com/v1/projects/{}:commit",
                self.project_id
            ))
            .bearer_auth(&self.access_token)
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
