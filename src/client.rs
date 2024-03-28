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
    client: Client,
    headers: HeaderMap,
    tenant: String,
    database: String,
}

impl ChromaClient {
    /// Creates a new ChromaClient instance.
    pub fn new(params: ChromaClientParams) -> Self {
        let http = if params.ssl { "https" } else { "http" };
        let settings = params.settings.unwrap_or(Settings::default());

        ChromaClient {
            path: format!("{}://{}:{}", http, params.host, params.port),
            client: Client::new(),
            headers: params.headers.unwrap_or(HeaderMap::new()),
            tenant: settings.tenant,
            database: settings.database,
        }
    }

    /// Get the current time in nanoseconds since epoch. Used to check if the server is alive.
    pub async fn heartbeat(&self) -> Result<u64, ChromaClientError> {
        let res = self
            .client
            .get(&format!("{}/api/v1/heartbeat", self.path))
            .headers(self.headers.clone())
            .send()
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
                ("tenant", self.tenant.clone()),
                ("database", self.database.clone()),
            ],
        )
        .map_err(ChromaClientError::UrlParseError)?;

        let mut headers = self.headers.clone();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let request_body = CreateCollectionRequest {
            name: name.to_string(),
            metadata: Some(metadata).unwrap_or(None),
            get_or_create: false,
        };

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await
            .map_err(ChromaClientError::RequestError)?;

        let response_text = response
            .text()
            .await
            .map_err(|e| ChromaClientError::ResponseError(e))?;

        let response_json: CreateCollectionResponse = serde_json::from_str(&response_text)
            .map_err(|e| ChromaClientError::ResponseParseError(e))?;

        Ok(Collection {
            name: response_json.name,
            id: response_json.id,
            metadata: response_json.metadata,
        })
    }

    /// Get a collection with the given name.
    pub async fn get_collection(&self, name: &str) -> Result<Collection, ChromaClientError> {
        let url = Url::parse_with_params(
            &format!("{}/api/v1/collections/{}", self.path, name),
            &[
                ("tenant", self.tenant.clone()),
                ("database", self.database.clone()),
            ],
        )
        .map_err(ChromaClientError::UrlParseError)?;

        let mut headers = self.headers.clone();
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(ChromaClientError::RequestError)?;

        let response_text = response
            .text()
            .await
            .map_err(|e| ChromaClientError::ResponseError(e))?;

        let response_json: Collection = serde_json::from_str(&response_text)
            .map_err(|e| ChromaClientError::ResponseParseError(e))?;

        Ok(response_json)
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
                ("tenant", self.tenant.clone()),
                ("database", self.database.clone()),
            ],
        )
        .map_err(ChromaClientError::UrlParseError)?;

        let mut headers = self.headers.clone();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let request_body = CreateCollectionRequest {
            name: name.to_string(),
            metadata: Some(metadata).unwrap_or(None),
            get_or_create: true,
        };

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await
            .map_err(ChromaClientError::RequestError)?;

        let response_text = response
            .text()
            .await
            .map_err(|e| ChromaClientError::ResponseError(e))?;

        let response_json: CreateCollectionResponse = serde_json::from_str(&response_text)
            .map_err(|e| ChromaClientError::ResponseParseError(e))?;

        Ok(Collection {
            name: response_json.name,
            id: response_json.id,
            metadata: response_json.metadata,
        })
    }

    /// Delete a collection with the given name.
    pub async fn delete_collection(&self, name: &str) -> Result<(), ChromaClientError> {
        let url = format!(
            "{}/api/v1/collections/{}?tenant={}&database={}",
            self.path, name, self.tenant, self.database
        );

        let mut headers = self.headers.clone();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

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
            Err(ChromaClientError::ResponseStatusError(error_message))
        }
    }

    /// List all collections.
    pub async fn list_collections(&self) -> Result<Vec<Collection>, ChromaClientError> {
        let url = format!(
            "{}/api/v1/collections?tenant={}&database={}",
            self.path, self.tenant, self.database
        );

        let mut headers = self.headers.clone();
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(ChromaClientError::RequestError)?;

        if response.status().is_success() {
            let response_text = response
                .text()
                .await
                .map_err(|e| ChromaClientError::ResponseError(e))?;

            let response_json: ListCollectionsResponse = serde_json::from_str(&response_text)
                .map_err(|e| ChromaClientError::ResponseParseError(e))?;

            Ok(response_json)
        } else {
            let error_message = format!(
                "Failed to list collections with status code: {}",
                response.status()
            );
            Err(ChromaClientError::ResponseStatusError(error_message))
        }
    }

    /// Resets the database. This will delete all collections and entries.
    pub async fn reset(&self) -> Result<(), ChromaClientError> {
        let url = format!("{}/api/v1/reset", self.path);

        let mut headers = self.headers.clone();
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        let response = self
            .client
            .post(url)
            .headers(headers)
            .send()
            .await
            .map_err(ChromaClientError::RequestError)?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_message = format!(
                "Failed to reset with status code: {} - make sure `ALLOW_RESET=TRUE`",
                response.status()
            );
            Err(ChromaClientError::ResponseStatusError(error_message))
        }
    }
}

/// The parameters to create a new client.
pub struct ChromaClientParams {
    pub host: String,
    pub port: String,
    pub ssl: bool,
    pub headers: Option<HeaderMap>,
    pub settings: Option<Settings>,
}

/// The settings for a client.
pub struct Settings {
    pub tenant: String,
    pub database: String,
}

impl ChromaClientParams {
    /// The default parameters for a Chroma Client.
    pub fn default() -> Self {
        ChromaClientParams {
            host: String::from("localhost"),
            port: String::from("8000"),
            ssl: false,
            headers: None,
            settings: Some(Settings::default()),
        }
    }
}

impl Default for ChromaClientParams {
    fn default() -> Self {
        ChromaClientParams::default()
    }
}

impl Settings {
    /// The default settings for a Chroma Client.
    pub fn default() -> Self {
        Settings {
            tenant: String::from("default_tenant"),
            database: String::from("default_database"),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::default()
    }
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

// No need to derive Deserialize for a Vec
type ListCollectionsResponse = Vec<Collection>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn heartbeat() {
        let client = ChromaClient::new(ChromaClientParams::default());

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
        let client = ChromaClient::new(ChromaClientParams::default());

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
        let client = ChromaClient::new(ChromaClientParams::default());

        let default = Collection {
            name: "default-collection".into(),
            id: "null".into(),
            metadata: None,
        };

        let new_collection = match client
            .get_or_create_collection("john-doe-g-or-c-collection", None)
            .await
        {
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
