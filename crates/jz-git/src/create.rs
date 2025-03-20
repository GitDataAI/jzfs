use crate::GitParam;
use git2::Repository;

impl GitParam {
    pub fn repo_create(&self, force: bool) -> anyhow::Result<()> {
        if !self.root.exists() {
            std::fs::create_dir_all(&self.root)?;
        }
        let path = self.root.join(self.uid.clone());
        if path.exists() && !force {
            return Err(anyhow::anyhow!("repo uid already exists"));
        }
        if path.exists() && force {
            std::fs::remove_dir_all(&path)?;
        }
        Repository::init_bare(&path)?;
        Ok(())
    }
}

#[test]
fn test_create_repo() {
    use std::path::PathBuf;
    let root = PathBuf::from("./test/jz-git");
    let uid = "test";
    let param = GitParam {
        root,
        uid: uid.to_string(),
        repo: None,
    };
    param.repo_create(true).unwrap();
}
