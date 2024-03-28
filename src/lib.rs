//! # Chromadb-rs
//!
//! The unofficial lib for [Chroma](https://docs.trychroma.com/) in rust. Chroma-rs uses endpoints of the [chroma db backend](https://docs.trychroma.com/api#backend-api).
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
//! let client = ChromaClient::new(ChromaClientParams {
//!     host: "localhost".to_string(),
//!     port: "8000".to_string(),
//!     ssl: false,
//! });
//! ```
//!

pub mod client;
pub mod collection;
pub mod error;
