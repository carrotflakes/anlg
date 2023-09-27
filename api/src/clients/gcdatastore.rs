use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Client {
    reqwest: reqwest::Client,
    url: String,
    project_id: String,
    token_getter: TokenGetter,
}

impl Client {
    pub fn new(url: String, project_id: String, token_getter: TokenGetter) -> Self {
        Client {
            reqwest: reqwest::Client::new(),
            url,
            project_id,
            token_getter,
        }
    }

    pub async fn run_query<T: Serialize>(&self, query: &T) -> Result<Vec<(Path, Value)>> {
        let access_token = self.token_getter.get().await;
        let client = &self.reqwest;
        let res = client
            .post(&format!(
                "{}/v1/projects/{}:runQuery",
                self.url, self.project_id
            ))
            .bearer_auth(&access_token)
            .json(&query)
            .send()
            .await
            .unwrap();
        let text = res.text().await.unwrap();
        let Ok(res) = serde_json::from_str::<QueryResult>(&text) else {
            return Err(format!("parse failed: {}", text).into());
        };
        // log::debug!("Datastore query response: {:?}", res);
        let entities = res
            .batch
            .entity_results
            .into_iter()
            .map(|er| (er.entity.key.path[0].clone(), er.entity.properties))
            .collect();
        Ok(entities)
    }

    pub async fn commit(&self, commit: Commit) -> Result<MutationResult> {
        let mutation = match commit {
            Commit::Insert { kind, properties } => {
                json!({
                    "insert": {
                        "key": {
                            "partitionId": {
                                "namespaceId": ""
                            },
                            "path": [
                                {
                                    "kind": kind
                                }
                            ]
                        },
                        "properties": properties
                    }
                })
            }
            Commit::Update {
                kind,
                id,
                properties,
            } => {
                json!({
                    "update": {
                        "key": {
                            "partitionId": {
                                "namespaceId": ""
                            },
                            "path": [
                                {
                                    "kind": kind,
                                    "id": id
                                }
                            ]
                        },
                        "properties": properties
                    }
                })
            }
            Commit::Delete { kind, id } => {
                json!({
                    "delete": {
                        "partitionId": {
                            "namespaceId": ""
                        },
                        "path": [
                            {
                                "kind": kind,
                                "id": id
                            }
                        ]
                    }
                })
            }
        };
        let client = &self.reqwest;
        let access_token = self.token_getter.get().await;
        let res = client
            .post(&format!(
                "{}/v1/projects/{}:commit",
                self.url, self.project_id
            ))
            .bearer_auth(&access_token)
            .json(&json!({
                "mode": "NON_TRANSACTIONAL",
                "mutations": [
                    mutation
                ]
            }))
            .send()
            .await
            .unwrap();
        // dbg!(res.json::<Value>().await.unwrap());
        let text = res.text().await.unwrap();
        let Ok(res) = serde_json::from_str::<MutationResult>(&text) else {
            log::info!("Request: {:?}", serde_json::to_string(&mutation).unwrap());
            return Err(format!("parse failed: {}", text).into());
        };
        Ok(res)
    }
}

pub enum Commit {
    Insert {
        kind: String,
        properties: Value,
    },
    Update {
        kind: String,
        id: String,
        properties: Value,
    },
    Delete {
        kind: String,
        id: String,
    },
}

#[derive(Debug, Deserialize)]
struct QueryResult {
    batch: Batch,
}

#[derive(Debug, Deserialize)]
struct Batch {
    #[serde(rename = "entityResultType")]
    entity_result_type: String,
    #[serde(rename = "entityResults", default)]
    entity_results: Vec<EntityResult>,
}

#[derive(Debug, Deserialize)]
struct EntityResult {
    entity: Entity,
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    key: Key,
    properties: Value,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    #[serde(rename = "partitionId")]
    pub partition_id: Value,
    pub path: Vec<Path>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Path {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct MutationResult {
    #[serde(rename = "indexUpdates")]
    pub index_updates: Option<i32>,
    #[serde(rename = "mutationResults")]
    pub mutation_results: Vec<MutationResultItem>,
}

#[derive(Debug, Deserialize)]
pub struct MutationResultItem {
    pub key: Option<Key>,
    pub version: String,
}

pub enum TokenGetter {
    Dummy,
    ServiceAccount(super::gcp::TokenGetter),
    ACD,
}

impl TokenGetter {
    pub async fn get(&self) -> String {
        match self {
            TokenGetter::Dummy => "dummy".to_owned(),
            TokenGetter::ServiceAccount(gcp) => gcp.get().await,
            TokenGetter::ACD => super::gcp::get_meta_data().await.access_token,
        }
    }
}
