use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub id: String,
    pub metadata: Option<Value>,
}

impl Collection {
    pub fn new(name: String, metadata: Option<Value>) -> Self {
        Collection {
            name,
            id: String::new(),
            metadata,
        }
    }

    pub fn with_id(name: String, id: String, metadata: Option<Value>) -> Self {
        Collection { name, id, metadata }
    }
}
