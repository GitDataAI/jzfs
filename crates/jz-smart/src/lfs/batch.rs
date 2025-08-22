use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LfsBatchObject {
    pub oid: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize)]
struct LfsBatchRef {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct Root {
    pub operation: String,
    pub transfers: Vec<String>,
    #[serde(rename = "ref")]
    pub r#ref: LfsBatchRef,
    pub objects: Vec<LfsBatchObject>,
    pub hash_algo: String,
}