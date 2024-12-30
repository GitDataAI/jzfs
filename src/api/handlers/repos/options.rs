use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryList {
    pub page: u64,
    pub size: u64,
}

#[derive(Debug, Deserialize)]
pub struct RepoPath {
    pub path: String,
    pub size_limit: usize,
    pub page: usize,
}
