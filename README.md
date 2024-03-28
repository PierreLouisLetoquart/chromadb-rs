# Chroma Rust Library

This is a Rust library for interacting with the ChromaDB vector database. It's intended for learning and educational purposes. For a more advanced library, please check out [chromadb](https://crates.io/crates/chromadb).

## 1. Running the Backend

Here's how to run the ChromaDB backend using Docker:

**default configuration:**

```bash
docker pull chromadb/chroma
docker run -p 8000:8000 chromadb/chroma
```

**with auth using token & persistant storage:**

```bash
docker pull chromadb/chroma
docker run \
	-p 8000:8000 \
	-e chroma_server_auth_credentials_provider="chromadb.auth.token.tokenconfigserverauthcredentialsprovider" \
	-e chroma_server_auth_provider="chromadb.auth.token.tokenauthserverprovider" \
	-e chroma_server_auth_token_transport_header="x_chroma_token" \
	-e chroma_server_auth_credentials="pilou2024" \
	-v /path/to/persistent/storage/:/chroma/chroma \
	chromadb/chroma
```

## 2. Default Client

> This asynchronous example uses [Tokio](https://docs.rs/tokio/latest/tokio/) crate.

Here's a basic example of how to create a default client:

```rust
use chromadb_rs::client::{ChromaClient, ChromaClientParams};

let client = ChromaClient::new(ChromaClientParams::default());

let hb = client.heartbeat().await?;

println!("{hb}");
```

## 3. Advanced Client

> This asynchronous example uses [Tokio](https://docs.rs/tokio/latest/tokio/) crate.

For more advanced usage, you can create a client with custom parameters:

```rust
let mut hmap = HeaderMap::new();
hmap.insert("X-Chroma-Token", "test-token".parse().unwrap());

let client = ChromaClient::new(ChromaClientParams {
    host: "localhost".to_string(),
    port: "8000".to_string(),
    ssl: false,
    headers: Some(hmap),
});
```

## 4. Create and Delete Collections

Here's how to create and delete a collection:

```rust
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
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
