# Chroma rs

There is a more advanced library available called [chromadb](https://crates.io/crates/chromadb). This one is currently for learning purpose.

## Usage example

Let's run a Chroma db backend using docker. We'll also add authentification using static token and persistant storage!

```bash
docker run \
	-p 8000:8000 \
	-e CHROMA_SERVER_AUTH_CREDENTIALS_PROVIDER="chromadb.auth.token.TokenConfigServerAuthCredentialsProvider" \
	-e CHROMA_SERVER_AUTH_PROVIDER="chromadb.auth.token.TokenAuthServerProvider" \
	-e CHROMA_SERVER_AUTH_TOKEN_TRANSPORT_HEADER="X_CHROMA_TOKEN" \
	-e CHROMA_SERVER_AUTH_CREDENTIALS="pilou2024" \
	-v /path/to/persistant/storage/:/chroma/chroma \
	chromadb/chroma
```

Then let's write a basic script which create a delete a collection

```rust
use chromadb_rs::client::{ChromaClient, ChromaClientParams};
use reqwest::header::HeaderMap;
use std::{collections::HashMap, error::Error, result::Result};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
                    "my first collection into a vector db".to_string(),
                ),
                (   
                    "other".to_string(), 
                    "metadata are hard to deserialize...".to_string()
                ),
            ])),
        )
        .await?;

    println!("{:?}", new_collection);

    let _ = client.delete_collection("crea").await?;

    Ok(())
}
```

