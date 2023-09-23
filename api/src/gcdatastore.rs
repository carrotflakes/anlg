use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

    pub async fn run_query<T: Serialize>(&self, query: &T) -> Vec<(Path, Value)> {
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
            panic!("parse failed: {}", text)
        };
        res.batch
            .entity_results
            .into_iter()
            .map(|er| (er.entity.key.path[0].clone(), er.entity.properties))
            .collect()
    }

    pub async fn commit(&self, commit: Commit) {
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
        dbg!(res.json::<Value>().await.unwrap());
    }
}

pub enum Commit {
    Insert { kind: String, properties: Value },
    Delete { kind: String, id: String },
}

#[derive(Deserialize)]
struct QueryResult {
    batch: Batch,
}

#[derive(Deserialize)]
struct Batch {
    #[serde(rename = "entityResultType")]
    entity_result_type: String,
    #[serde(rename = "entityResults", default)]
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
    path: Vec<Path>,
}

#[derive(Clone, Deserialize)]
pub struct Path {
    pub kind: String,
    pub id: String,
}

pub enum TokenGetter {
    Dummy,
    ServiceAccount(crate::gcp::TokenGetter),
    ACD,
}

impl TokenGetter {
    pub async fn get(&self) -> String {
        match self {
            TokenGetter::Dummy => "dummy".to_owned(),
            TokenGetter::ServiceAccount(gcp) => gcp.get().await,
            TokenGetter::ACD => crate::gcp::get_meta_data().await.access_token,
        }
    }
}
