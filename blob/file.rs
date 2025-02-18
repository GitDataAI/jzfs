use crate::blob::GitBlob;
use git2::Oid;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;


impl GitBlob {
    pub fn file(&self, path: String, sha: String) -> io::Result<Vec<u8>> {
        let oid = Oid::from_str(&sha)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to parse sha"))?;
        let commit = self.repository.find_commit(oid)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find commit"))?;
        let tree = commit.tree()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find tree"))?;
        let path = PathBuf::from_str(&path)
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x.to_string()))?;
        let blob = tree.get_path(&path)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find blob 3"))?;
        let blob = blob.to_object(&self.repository)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find blob 2"))?;
        let blob = blob.as_blob()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to find blob 1"))?;
        let content = blob.content();
        Ok(content.to_vec())
    }
    pub fn file_readme(&self) -> io::Result<Vec<u8>> {
           if let Ok(tree) = self.repository.find_tree(self.repository.head()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find head"))?
            .peel_to_tree().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find tree"))?
            .id()) {
            if let Ok(blob) = tree.get_path(&PathBuf::from_str("README.md").map_err(|x| io::Error::new(io::ErrorKind::Other, x.to_string()))?) {
                if let Ok(blob) = blob.to_object(&self.repository) {
                    if let Some(blob) = blob.as_blob() {
                        return Ok(blob.content().to_vec());
                    }
                }
            }
        }
        Err(io::Error::new(io::ErrorKind::Other, "Failed to find README.md"))
    }
}

#[cfg(test)]
mod tests {
    use crate::blob::GitBlob;
    use std::io;

    #[test]
    fn test_file() -> io::Result<()> {
        let blob = GitBlob::new("/home/zhenyi/文档/actix-web".into()).unwrap();
        let file = blob.file("justfile".to_string(), "a4eaa7f0bb963ec6bc67aaadafdda2d638f9ba41".to_string());
        dbg!(file);
        Ok(())
    }
}
