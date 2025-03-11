use crate::blob::GitBlob;
use git2::Oid;
use std::io;
use crate::product::zip::compress_directory_to_zip;

impl GitBlob {
    pub fn post_product(&self,hash: Oid) -> io::Result<()> {
        let tmp = tempfile::tempdir()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let path = tmp.path().to_path_buf();
        let clone = git2::Repository::clone(self.root.to_str().ok_or(io::ErrorKind::Other)?, tmp.path())
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, e)
            })?;
        if std::fs::metadata(self.root.join("product")).is_err() {
            std::fs::create_dir_all(self.root.join("product"))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }
        let commit = clone.find_commit(hash)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        clone.checkout_tree(&commit.into_object(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        clone.set_head_detached(hash)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        clone.cleanup_state()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        copy_dir::copy_dir(
            path,
            self.root.join("product").join(hash.to_string())
        )?;
        compress_directory_to_zip(
            self.root.join("product").join(hash.to_string()),
            self.root.join("product").join(hash.to_string()).with_extension("zip")
        )?;
        tmp.close()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use crate::blob::GitBlob;

    #[test]
    fn test_post() -> io::Result<()> {
        let blob = GitBlob::new("/home/zhenyi/文档/zino".into()).unwrap();
        let file = blob.post_product("6674d18880203c58dca81b0470e7e9d3e9c5ae73".parse().unwrap());
        dbg!(file);
        Ok(())
    }
}