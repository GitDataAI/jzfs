use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct RepositoryInit {
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub project: Vec<Uuid>,
    pub is_private: bool,
    pub topic: Vec<String>,
    pub rtype: String,
    pub storage: Uuid,
    pub license: Option<String>,
}
