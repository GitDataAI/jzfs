use git2::{Branch, BranchType, Repository};
use crate::api::dto::repo_dto::RepoTree;
use crate::git::commits::GitCommits;

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
    pub fn tree(&self, branch: Branch, commit_ids: Option<String>) -> anyhow::Result<RepoTree>{
        let refs = branch.into_reference();
        let tree = refs.peel_to_commit();
        if tree.is_err(){
            return Err(tree.err().unwrap().into());
        }
        let mut commit_id = tree?;
        let commit = if let Some(id) = commit_ids{
            loop {
                if commit_id.id().to_string() == id{
                    break commit_id;
                }else {
                    match commit_id.parent(0){
                        Ok(parent) => commit_id = parent,
                        Err(_) => break refs.peel_to_commit()?
                    }
                }
            }
        }else { 
            commit_id
        };
        GitCommits::build_tree(&self.repo, commit.tree()?.id(), "".to_string())
    }
}

