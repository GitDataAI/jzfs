use git2::{Branch, BranchType, Repository};

pub struct GitBranch{
    repo: Repository
}

impl GitBranch{
    pub fn new(repo: Repository) -> Self{
        Self{
            repo
        }
    }
    pub fn branchs(&self) -> anyhow::Result<Vec<Branch>>{
        let branchs = match self.repo.branches(Some(BranchType::Local)){
            Ok(branchs) => branchs,
            Err(e) => return Err(e.into())
        };
        let mut result = Vec::new();
        for item in branchs{
            if let Ok((branch, _)) = item{
                result.push(branch);
            }
        }
        Ok(result)
    }
}

