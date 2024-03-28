//! # Chromadb-rs
//!
//! Currently the lib is a work in progress... check out the [github repo](https://github.com/PierreLouisLetoquart/chroma-rs) to contribute.
//!
//! ## Chroma client
//!
//! **Creates a new ChromaClient instance:**
//!
//! ```rust
//! use chromadb_rs::client::{ChromaClient, ChromaClientParams};
//!
//! let client = ChromaClient::new(ChromaClientParams::default());
//! ```
//!
//! **Creates a new ChromaClient instance with params:**
//!
//! ```rust
//! use chromadb_rs::client::{ChromaClient, ChromaClientParams};
//! use reqwest::header::HeaderMap;
//!
//! let mut hmap = HeaderMap::new();
//! hmap.insert("X-Chroma-Token", "test-token".parse().unwrap());
//!
//! let client = ChromaClient::new(ChromaClientParams {
//!     host: "localhost".to_string(),
//!     port: "8000".to_string(),
//!     ssl: false,
//!     headers: Some(hmap),
//! });
//! ```

pub mod client;
pub mod collection;
pub mod error;
