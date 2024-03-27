use serde::{Deserialize, Serialize};
use std::error::Error;

/// Chroma Client instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaClient {
    path: String,
}

impl ChromaClient {
    /// Creates a new ChromaClient instance.
    pub fn new(params: ChromaClientParams) -> Self {
        let http = if params.ssl { "https" } else { "http" };
        ChromaClient {
            path: format!("{}://{}:{}", http, params.host, params.port),
        }
    }

    /// Returns a heartbeat from the Chroma API.
    pub async fn heartbeat(&self) -> Result<u64, Box<dyn Error>> {
        let res = reqwest::get(&format!("{}/api/v1/heartbeat", self.path))
            .await?
            .text()
            .await?;
        let body_json: HeartbeatResponse = serde_json::from_str(&res)?;
        Ok(body_json.nanosecond_heartbeat)
    }
}

/// The parameters for creating a new client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaClientParams {
    pub host: String,
    pub port: String,
    pub ssl: bool,
}

/// The returned type of a heartbeat from the Chroma API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    #[serde(rename = "nanosecond heartbeat")]
    nanosecond_heartbeat: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn heart_beats() {
        let client = ChromaClient::new(ChromaClientParams {
            host: "localhost".to_string(),
            port: "8000".to_string(),
            ssl: false,
        });

        let default: u64 = 0;
        let hb = client.heartbeat().await.unwrap_or(default);

        // YOU NEED TO HAVE YOUR CHROMA INSTANCE UP TO USE assert_ne!(...)
        assert_ne!(hb, default);
    }
}
