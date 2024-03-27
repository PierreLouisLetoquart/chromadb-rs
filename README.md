# Chroma rs

There is a more advanced library available called [chromadb](https://crates.io/crates/chromadb). This one is currently for learning purpose.

## Chroma client

**Creates a new ChromaClient instance:**

```rust
use chromadb_rs::client::{ChromaClient, ChromaClientParams};

let client = ChromaClient::new(ChromaClientParams {
    host: "localhost".to_string(),
    port: "8000".to_string(),
    ssl: false,
});
```

