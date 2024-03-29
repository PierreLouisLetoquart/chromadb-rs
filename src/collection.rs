use core::fmt;
use std::{collections::HashSet, error::Error, marker::PhantomData};

use serde::{
    de::{DeserializeOwned, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
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

    pub fn add(&self, params: AddParams) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }

    pub fn get<T: DeserializeOwned>(&self, params: GetParams) -> Result<Vec<T>, Box<dyn Error>> {
        unimplemented!();
    }

    pub fn query<T: DeserializeOwned>(
        &self,
        params: QueryParams,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ID(pub String);

// Add function parameters
// ========================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding(pub Vec<f32>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata(pub HashSet<(String, String)>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document(pub Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance(pub f32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddParams {
    pub ids: Vec<ID>,
    pub embeddings: Option<Vec<Embedding>>,
    pub metadatas: Option<Vec<Metadata>>,
    pub documents: Option<Vec<Document>>,
}

// Get function parameters
// ========================
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Include {
    Embeddings,
    Metadatas,
    Documents,
    Distances,
}

// TODO: complete and double check `Where` and `WhereDocument` types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Where {
    pub fields: Vec<(String, serde_json::Value)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhereDocument {
    pub operator: String,
    pub field: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParams {
    pub ids: Option<Vec<ID>>,
    pub where_: Option<Where>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub where_document: Option<WhereDocument>,
    pub include: Vec<Include>,
}

impl Default for GetParams {
    fn default() -> Self {
        GetParams {
            ids: None,
            where_: None,
            limit: None,
            offset: None,
            where_document: None,
            include: vec![Include::Metadatas, Include::Documents],
        }
    }
}

// Query function parameters
// ==========================
pub struct QueryParams {
    pub query_embeddings: Option<Vec<Embedding>>,
    pub query_texts: Option<Vec<Document>>,
    pub n_results: i32,
    pub where_: Option<serde_json::Value>,
    pub where_document: Option<serde_json::Value>,
    pub include: Vec<Include>,
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams {
            query_embeddings: None,
            query_texts: None,
            n_results: 10,
            where_: None,
            where_document: None,
            include: vec![Include::Metadatas, Include::Documents, Include::Distances],
        }
    }
}
