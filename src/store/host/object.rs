use crate::store::dto::ObjectFile;
use crate::store::host::GitLocal;

impl GitLocal {
    pub fn object_tree(&self, branch: String) -> anyhow::Result<Vec<ObjectFile>>{
        let head = self.repo.find_branch(branch.as_str(), git2::BranchType::Local);
        if head.is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let head = head?.into_reference();
        let head_commit = self.repo.find_commit(head.target().unwrap());
        let tree = head_commit?.tree()?;
        let mut clo = vec![];
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            clo.push(ObjectFile{
                root: root.to_string(),
                name: entry.name().unwrap_or("N/A").to_string(),
                hash: entry.id().to_string(),
            });
            0
        })?;
        Ok(clo)
    }
    pub fn object_hash(&self, branch: String, hash: String) -> anyhow::Result<Vec<ObjectFile>>{
        let head = self.repo.find_branch(branch.as_str(), git2::BranchType::Local);
        if head.is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let head = head?.into_reference();
        let head_commit = self.repo.find_commit(head.target().unwrap());
        let tree = head_commit?.tree()?;
        let mut clo = vec![];
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            if entry.id().to_string() == hash{
                clo.push(ObjectFile{
                    root: root.to_string(),
                    name: entry.name().unwrap_or("N/A").to_string(),
                    hash: entry.id().to_string(),
                })
            }
            0
        })?;
        Ok(clo)
    }
    pub fn object_file(&self, branch: String, hash: String, files: String) -> anyhow::Result<Vec<u8>>{
        let head = self.repo.find_branch(branch.as_str(), git2::BranchType::Local);
        if head.is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let head = head?.into_reference();
        let head_commit = self.repo.find_commit(head.target().unwrap());
        let tree = head_commit?.tree()?;
        let mut id: Option<git2::Oid> = None;
        let _ = tree.walk(git2::TreeWalkMode::PreOrder, |_, entry| {
            if entry.id().to_string() == hash {
                id = Some(entry.id());
            }
            0
        });
        if id.is_none(){
            return Err(anyhow::anyhow!("File not found"))
        }
        let tree = self.repo.find_object(id.unwrap(), None)?.into_tree();
        if tree.is_err(){
            return Err(anyhow::anyhow!("Tree not found"))
        }
        let tree = tree.unwrap();
        let obj = tree.get_path((&files.as_str()).as_ref());
        if obj.is_err(){
            return Err(anyhow::anyhow!("File not found"))
        }
        
        let obj = obj?.to_object(&self.repo)?;
        let obj = obj.into_blob();
        let content = obj.unwrap().content().to_vec();
        Ok(content)
    }
}