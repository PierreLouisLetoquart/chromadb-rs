# Chroma rs

There is a more advanced library available called [chromadb](https://crates.io/crates/chromadb). This one is currently for learning purpose.

## Basic example

```rust
use chromadb_rs::client::{ChromaClient, ChromaClientParams};
use std::{collections::HashMap, error::Error, result::Result};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ChromaClient::new(ChromaClientParams {
        host: "localhost".to_string(),
        port: "8000".to_string(),
        ssl: false,
    });

    let hb = client.heartbeat().await?;

    println!("{hb}");

    let new_collection = client.create_collection("crea", None).await?;

    println!("{:?}", new_collection);

    let _ = client.delete_collection(new_collection.name).await?;

    Ok(())
}
```
