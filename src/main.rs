use chromadb_rs::client::{ChromaClient, ChromaClientParams};
use reqwest::header::HeaderMap;
use std::{collections::HashMap, error::Error, result::Result};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // headers are optional, here is an example with token auth.
    // checkout https://docs.trychroma.com/usage-guide#static-api-token-authentication for more!
    let mut hmap = HeaderMap::new();
    hmap.insert("X-Chroma-Token", "test-token".parse().unwrap());

    let client = ChromaClient::new(ChromaClientParams {
        host: "localhost".to_string(),
        port: "8000".to_string(),
        ssl: false,
        headers: Some(hmap),
    });

    let hb = client.heartbeat().await?;

    println!("{hb}");

    let new_collection = client
        .create_collection(
            "crea",
            Some(HashMap::from([
                (
                    "description".to_string(),
                    "testing collection creation".to_string(),
                ),
                ("wtf".to_string(), "hard to deserialize...".to_string()),
            ])),
        )
        .await?;

    println!("{:?}", new_collection);

    let _ = client.delete_collection("crea").await?;

    Ok(())
}
