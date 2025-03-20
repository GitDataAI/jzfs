use git2::Repository;
use std::path::PathBuf;

pub mod branches;
pub mod commits;
pub mod create;
pub mod delete;
pub mod list;
pub mod tree;
pub mod tree_dir;
pub mod blob;
pub mod clone;

pub struct GitParam {
    pub root: PathBuf,
    pub uid: String,
    pub repo: Option<Repository>,
}

impl GitParam {
    pub fn new(root: PathBuf, uid: String) -> anyhow::Result<GitParam> {
        Ok(GitParam {
            root,
            uid,
            repo: None,
        })
    }
    pub fn repo(&mut self) -> anyhow::Result<&Repository> {
        if self.repo.is_none() {
            self.repo = Some(Repository::open(self.root.join(&self.uid))?);
        }
        Ok(self.repo.as_ref().unwrap())
    }
    pub fn dir(&self) -> PathBuf {
        self.root.join(&self.uid)
    }
}

#[test]
pub fn test() {}
