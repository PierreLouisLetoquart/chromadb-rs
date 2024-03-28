use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub id: String,
    pub metadata: Option<HashMap<String, String>>,
}

impl Collection {
    pub fn new(name: String, metadata: Option<HashMap<String, String>>) -> Self {
        Collection {
            name,
            id: String::new(),
            metadata,
        }
    }

    pub fn with_id(name: String, id: String, metadata: Option<HashMap<String, String>>) -> Self {
        Collection { name, id, metadata }
    }
}
