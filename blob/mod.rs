use git2::Repository;
use std::path::PathBuf;

pub struct GitBlob {
    pub root: PathBuf,
    pub repository: Repository,
}

impl GitBlob {
    pub fn new(root: PathBuf) -> Result<Self, git2::Error> {
        let repository = Repository::open(root.clone())?;
        Ok(Self { root, repository })
    }
}
pub mod blob;
pub mod tree;
pub mod file;