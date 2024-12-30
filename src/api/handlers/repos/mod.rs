use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod blob;
pub mod branchs;
pub mod commits;
pub mod options;
pub mod repos;
pub mod star;
pub mod tree;
pub mod watch;
pub mod check;


#[derive(Deserialize,Serialize)]
pub struct RepoCreateOwnerList{
    pub uid: Uuid,
    pub name: String,
    pub group: String,
    pub avatar_url: String,
}