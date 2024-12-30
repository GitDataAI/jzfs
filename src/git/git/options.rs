use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Branchs {
    pub name: String,
    pub head: String,
    pub local: bool,
    pub remote_url: Option<Vec<(String, String)>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Commits {
    pub hash_oid: String,
    pub msg: String,
    pub username: String,
    pub email: String,
    pub branchs: String,
    pub time: i64,
    pub tree: BlobTree,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BlobTree {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: usize,
    pub children: Vec<BlobTree>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BlobTreeMsg {
    pub name: String,
    pub path: String,
    pub msg: String,
    pub time: i64,
    pub is_dir: bool,
    pub size: usize,
    pub children: Vec<BlobTreeMsg>,
}
