use crate::GitParam;

impl GitParam {
    pub fn list(&self) -> anyhow::Result<Vec<String>> {
        let mut dir = std::fs::read_dir(self.root.clone())?;
        let mut ret = vec![];
        while let Some(Ok(entry)) = dir.next() {
            let path = entry.path();
            if path.is_dir() {
                ret.push(path.file_name().unwrap().to_string_lossy().to_string());
            }
        }
        Ok(ret)
    }
}

#[test]
fn test_list() {
    use crate::GitParam;
    use std::path::PathBuf;
    let root = PathBuf::from("./test/jz-git");
    let uid = "test.git";
    let param = GitParam {
        root,
        uid: uid.to_string(),
        repo: None,
    };
    param.repo_create(true).unwrap();

    assert_eq!(param.list().unwrap(), vec!["test.git".to_string()]);
}
