use git2::Repository;
use std::path::PathBuf;

pub struct GitBlob {
    pub root: PathBuf,
    pub bare_repository: Repository,
    pub repository: Repository,
}

impl GitBlob {
    pub fn new(root: PathBuf) -> Result<Self, git2::Error> {
        let bare_repository = Repository::open(root.join(".git"))?;
        let repository = Repository::open(root.clone())?;
        Ok(Self { root, repository, bare_repository })
    }
}
pub mod blob;
pub mod tree;
