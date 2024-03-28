# Chroma Rust Library

This is a Rust library for interacting with the ChromaDB vector database. It's intended for learning and educational purposes. For a more advanced library, please check out [chromadb](https://crates.io/crates/chromadb).

> The asynchronous example uses [Tokio](https://docs.rs/tokio/latest/tokio/) crate.

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

Here's a basic example of how to create a default client:

```rust
use chromadb_rs::client::{ChromaClient, ChromaClientParams};

let client = ChromaClient::new(ChromaClientParams::default());
```

## 3. Advanced Client

For more advanced usage, you can create a client with custom parameters:

```rust
let mut hmap = HeaderMap::new();
hmap.insert("X-Chroma-Token", "test-token".parse().unwrap());

let settings = Settings {
    tenant: "my-tenant".to_string(),
    database: "my-database".to_string(),
}

let client = ChromaClient::new(ChromaClientParams {
    host: "localhost".to_string(),
    port: "8000".to_string(),
    ssl: false,
    headers: Some(hmap),
    settings: Some(settings), // Some(Settings::default()) for default settings
});
```

## 4. Chroma client methods

- Heartbeat:

```rust
let hb = client.heartbeat().await?;
```

- Get all collections:

```rust
let collections = client.list_collections().await?;
```

- Create a collection without metadata:

```rust
let new_collection = client.create_collection("test-name", None).await?;
```

- Create a collection with metadata:

```rust
let mut metadata = HashMap::new();
metadata.insert("key1", "value1");
metadata.insert("key2", "value2");

let new_collection = client
    .create_collection("test-name", Some(metadata)).await?;
```

- Create a collection using get or create:

```rust
let new_collection = client.get_or_create_collection("test-name", None).await?;
```

- Get a collection:

```rust
let collection = client.get_collection("test-name").await?;
```

- Delete a collection:

```rust
let deleted_collection = client.delete_collection("test-name").await?;
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
