use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use uuid::Uuid;

pub static DATA_PATH:&'static str = "./data";

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct IDX{
    pub owner_id: Uuid,
    pub repo_uid: Uuid,
    pub object: HashMap<String, Files> // <(dir + name), idx>
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Files{
    pub dir: String,
    pub name: String,
    pub repo_idx: String,
    pub hash: String,
    pub size: usize,
    pub offset: usize,
}
pub trait RepoFileTrait{
    fn from_idx(value: IDX) -> anyhow::Result<Self> where Self: Sized;
    fn read(&mut self, offset: usize, size: usize) -> anyhow::Result<Vec<u8>>;
    fn write(&mut self, offset: usize, data: Vec<u8>) -> anyhow::Result<()>;
    fn clear(&mut self, offset: usize, size: usize) -> anyhow::Result<()>;
}
