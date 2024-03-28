use chromadb_rs::client::{ChromaClient, ChromaClientParams};
use std::{error::Error, result::Result};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ChromaClient::new(ChromaClientParams::default());

    let _ = client
        .create_collection("collection-1", None, None, None)
        .await?;

    let coll_2 = client
        .get_or_create_collection(
            "collection-2",
            Some(std::collections::HashMap::from([
                (
                    "description".to_string(),
                    "my first collection into a vector db".to_string(),
                ),
                (
                    "other-metadata".to_string(),
                    "testing get or create fn with metadata...".to_string(),
                ),
            ])),
            None,
            None,
        )
        .await?;

    let _ = client
        .create_collection("collection-3", None, None, None)
        .await?;

    println!("Example collection : {:?}", coll_2);

    let coll_list = client.list_collections(None, None).await?;
    assert_eq!(coll_list.len(), 3);

    client.delete_collection("collection-1", None, None).await?;
    client.delete_collection("collection-2", None, None).await?;

    let coll_list = client.list_collections(None, None).await?;
    assert_eq!(coll_list.len(), 1);

    Ok(())
}
