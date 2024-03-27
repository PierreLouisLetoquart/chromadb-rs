use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaClient {
    path: String,
}

impl ChromaClient {
    pub fn new(params: ChromaClientParams) -> Self {
        let http = if params.ssl { "https" } else { "http" };
        ChromaClient {
            path: format!("{}://{}:{}", http, params.host, params.port),
        }
    }

    pub async fn heartbeat(&self) -> Result<u64, Box<dyn Error>> {
        let res = reqwest::get(&format!("{}/api/v1/heartbeat", self.path))
            .await?
            .text()
            .await?;
        let body_json: HeartbeatResponse = serde_json::from_str(&res)?;
        Ok(body_json.nanosecond_heartbeat)
    }

    // TODO: change Vec<String> to the proper type etc etc...
    pub async fn list_collections(&self) -> Result<Vec<String>, Box<dyn Error>> {
        unimplemented!("Unable to list cols for now...");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaClientParams {
    pub host: String,
    pub port: String,
    pub ssl: bool,
}

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

        let error_val: u64 = 0;
        let hb = client.heartbeat().await.unwrap_or(error_val);

        // YOU NEED TO HAVE YOUR CHROMA INSTANCE UP!
        assert_ne!(hb, error_val);
    }
}
