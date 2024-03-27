use chromadb_rs::client::{ChromaClient, ChromaClientParams};
use std::{error::Error, result::Result, collections::HashMap};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ChromaClient::new(ChromaClientParams {
        host: "localhost".to_string(),
        port: "8000".to_string(),
        ssl: false,
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

    Ok(())
}
