use crate::GitParam;

impl GitParam {
    pub fn delete(&self) -> anyhow::Result<()> {
        let path = self.root.join(self.uid.clone());
        if path.exists() {
            std::fs::remove_dir_all(path)?;
        }
        Ok(())
    }
}

#[test]
fn test_repo_delete() {
    use std::path::PathBuf;
    let root = PathBuf::from("./test/jz-git");
    let uid = "test";
    let param = GitParam {
        root,
        uid: uid.to_string(),
        repo: None,
    };
    param.repo_create(true).unwrap();
    param.delete().unwrap();
    assert!(!PathBuf::from("./test/jz-git").join(uid).exists());
}
