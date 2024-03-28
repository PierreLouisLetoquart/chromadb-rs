use crate::collection::Collection;
use crate::error::ChromaClientError;
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

/// Chroma Client instance.
#[derive(Debug, Clone)]
pub struct ChromaClient {
    path: String,
    tenant: String,
    database: String,
    client: Client,
}

impl ChromaClient {
    /// Creates a new ChromaClient instance.
    pub fn new(params: ChromaClientParams) -> Self {
        let http = if params.ssl { "https" } else { "http" };
        ChromaClient {
            path: format!("{}://{}:{}", http, params.host, params.port),
            tenant: String::from("default_tenant"),
            database: String::from("default_database"),
            client: Client::new(),
        }
    }

    fn req_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        headers
    }

    /// Get the current time in nanoseconds since epoch. Used to check if the server is alive.
    pub async fn heartbeat(&self) -> Result<u64, ChromaClientError> {
        let res = reqwest::get(&format!("{}/api/v1/heartbeat", self.path))
            .await
            .map_err(|e| ChromaClientError::RequestError(e))?;

        let res_text = res
            .text()
            .await
            .map_err(|e| ChromaClientError::ResponseError(e))?;

        let body_json: HeartbeatResponse = serde_json::from_str(&res_text)
            .map_err(|e| ChromaClientError::ResponseParseError(e))?;

        Ok(body_json.nanosecond_heartbeat)
    }

    /// Create a new collection with the given name and metadata.
    pub async fn create_collection(
        &self,
        name: &str,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Collection, ChromaClientError> {
        let url = Url::parse_with_params(
            &format!("{}/api/v1/collections", self.path),
            &[
                ("tenant", self.tenant.as_str()),
                ("database", self.database.as_str()),
            ],
        )
        .map_err(ChromaClientError::UrlParseError)?;

        let headers = Self::req_headers();

        let request = CreateCollectionRequest {
            name: name.to_string(),
            metadata: Some(metadata).unwrap_or(None),
            get_or_create: false,
        };

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(ChromaClientError::RequestError)?;

        let response_text = response
            .text()
            .await
            .map_err(|e| ChromaClientError::ResponseError(e))?;

        let response_json: CreateCollectionResponse = serde_json::from_str(&response_text)
            .map_err(|e| ChromaClientError::ResponseParseError(e))?;

        // TODO: unwrap properly the metadata !
        Ok(Collection {
            name: response_json.name,
            id: response_json.id,
            metadata: None,
        })
    }

    /// Get or create a collection with the given name and metadata.
    pub async fn get_or_create_collection(
        &self,
        name: &str,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Collection, ChromaClientError> {
        let url = Url::parse_with_params(
            &format!("{}/api/v1/collections", self.path),
            &[
                ("tenant", self.tenant.as_str()),
                ("database", self.database.as_str()),
            ],
        )
        .map_err(ChromaClientError::UrlParseError)?;

        let headers = Self::req_headers();

        let request = CreateCollectionRequest {
            name: name.to_string(),
            metadata: Some(metadata).unwrap_or(None),
            get_or_create: true,
        };

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(ChromaClientError::RequestError)?;

        let response_text = response
            .text()
            .await
            .map_err(|e| ChromaClientError::ResponseError(e))?;

        let response_json: CreateCollectionResponse = serde_json::from_str(&response_text)
            .map_err(|e| ChromaClientError::ResponseParseError(e))?;

        // TODO: unwrap properly the metadata !
        Ok(Collection {
            name: response_json.name,
            id: response_json.id,
            metadata: None,
        })
    }

    /// Delete a collection with the given name.
    pub async fn delete_collection(&self, name: &str) -> Result<(), ChromaClientError> {
        let url = format!(
            "{}/api/v1/collections/{}?tenant={}&database={}",
            self.path, name, self.tenant, self.database
        );

        let headers = Self::req_headers();

        let response = self
            .client
            .delete(url)
            .headers(headers)
            .send()
            .await
            .map_err(ChromaClientError::RequestError)?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_message = format!(
                "Failed to delete collection with status code: {}",
                response.status()
            );
            Err(ChromaClientError::DeleteCollectionError(error_message))
        }
    }
}

/// The parameters to create a new client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaClientParams {
    pub host: String,
    pub port: String,
    pub ssl: bool,
}

#[derive(Serialize, Deserialize)]
struct HeartbeatResponse {
    #[serde(rename = "nanosecond heartbeat")]
    nanosecond_heartbeat: u64,
}

#[derive(Serialize, Deserialize)]
struct CreateCollectionRequest {
    name: String,
    metadata: Option<HashMap<String, String>>,
    get_or_create: bool,
}

#[derive(Serialize, Deserialize)]
struct CreateCollectionResponse {
    name: String,
    id: String,
    metadata: Option<Value>,
    tenant: String,
    database: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn heartbeat() {
        let client = ChromaClient::new(ChromaClientParams {
            host: "localhost".to_string(),
            port: "8000".to_string(),
            ssl: false,
        });

        let default: u64 = 0;
        let hb = match client.heartbeat().await {
            Ok(hb) => hb,
            Err(ChromaClientError::RequestError(e)) => {
                eprintln!("Error during heartbeat: {}", e);
                default
            }
            Err(e) => {
                eprintln!("Unexpected error during heartbeat: {}", e);
                default
            }
        };

        assert_ne!(hb, default);
    }

    #[tokio::test]
    async fn create_and_delete() {
        let client = ChromaClient::new(ChromaClientParams {
            host: "localhost".to_string(),
            port: "8000".to_string(),
            ssl: false,
        });

        let default = Collection {
            name: "default-collection".into(),
            id: "null".into(),
            metadata: None,
        };

        let new_collection = match client.create_collection("john-doe-collection", None).await {
            Ok(new_collection) => new_collection,
            Err(ChromaClientError::RequestError(e)) => {
                eprintln!("Error during create_collection: {}", e);
                default
            }
            Err(e) => {
                eprintln!("Unexpected error during create_collection: {}", e);
                default
            }
        };

        assert_eq!(new_collection.name, "john-doe-collection");

        match client.delete_collection(&new_collection.name).await {
            Ok(_) => {}
            Err(ChromaClientError::RequestError(e)) => {
                eprintln!("Error during delete_collection: {}", e);
            }
            Err(e) => {
                eprintln!("Unexpected error during delete_collection: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn get_or_create_and_delete() {
        let client = ChromaClient::new(ChromaClientParams {
            host: "localhost".to_string(),
            port: "8000".to_string(),
            ssl: false,
        });

        let default = Collection {
            name: "default-collection".into(),
            id: "null".into(),
            metadata: None,
        };

        let new_collection = match client.get_or_create_collection("john-doe-g-or-c-collection", None).await {
            Ok(new_collection) => new_collection,
            Err(ChromaClientError::RequestError(e)) => {
                eprintln!("Error during get_or_create_collection: {}", e);
                default
            }
            Err(e) => {
                eprintln!("Unexpected error during get_or_create_collection: {}", e);
                default
            }
        };

        assert_eq!(new_collection.name, "john-doe-g-or-c-collection");

        match client.delete_collection(&new_collection.name).await {
            Ok(_) => {}
            Err(ChromaClientError::RequestError(e)) => {
                eprintln!("Error during delete_collection: {}", e);
            }
            Err(e) => {
                eprintln!("Unexpected error during delete_collection: {}", e);
            }
        }
    }
}
