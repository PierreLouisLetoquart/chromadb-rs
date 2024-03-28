//! Welcome to the Chromadb-rs Rust Library! This library allows you to interact with the ChromaDB vector database in Rust.
//!
//! !!! Disclaimer : It's designed for learning and educational purposes. A more advanced library exists, check out [chromadb](https://crates.io/crates/chromadb).
//!
//! Here's a quick guide on how to use this library:
//!
//! 1. **Run the Backend:** You can run the ChromaDB backend using Docker. For default configuration, use `docker pull chromadb/chroma` and `docker run -p 8000:8000 chromadb/chroma`. For auth using token and persistent storage, checkout [this link](https://github.com/PierreLouisLetoquart/chromadb-rs?tab=readme-ov-file#1-running-the-backend).
//!
//! 2. **Create a Default Client:** You can create a default client like this: `let client = ChromaClient::new(ChromaClientParams::default());`.
//!
//! 3. **Create an Advanced Client:** For more advanced usage, you can create a client with custom parameters. Check out the example in the [README](https://github.com/PierreLouisLetoquart/chromadb-rs?tab=readme-ov-file#chroma-rust-library).
//!
//! 4. **Use Chroma Client Methods:** The library provides methods like heartbeat, list collections, create collection, get or create collection, get collection, and delete collection. Check out the examples in the [README](https://github.com/PierreLouisLetoquart/chromadb-rs?tab=readme-ov-file#chroma-rust-library).
//!
//! For more detailed examples and usage, please check out the [GitHub repository](https://github.com/chroma-core/chromadb-rs).
//!
//! We welcome contributions! If you'd like to contribute, please open an issue first to discuss your changes.
//!
//! This project is licensed under the [MIT License](https://choosealicense.com/licenses/mit/).
//!
//! Happy coding! 😊

pub mod client;
pub mod collection;
pub mod error;
