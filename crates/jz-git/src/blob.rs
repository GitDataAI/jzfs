use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use git2::Oid;
use crate::GitParam;

impl GitParam {
    pub fn blob(&mut self, sha: String, path: String) -> anyhow::Result<Vec<u8>> {
        let oid = Oid::from_str(&sha)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to parse sha"))?;
        let repo = self.repo()?;
        let commit = repo.find_commit(oid)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find commit"))?;
        let tree = commit.tree()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find tree"))?;
        let path = PathBuf::from_str(&path)
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x.to_string()))?;
        let blob = tree.get_path(&path)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find blob 3"))?;
        let blob = blob.to_object(&repo)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find blob 2"))?;
        let blob = blob.as_blob()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to find blob 1"))?;
        let content = blob.content();
        Ok(content.to_vec())
    }
}