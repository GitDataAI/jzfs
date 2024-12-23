use crate::api::dto::repo_dto::RepoTree;
use crate::git::commits::GitCommits;
use crate::metadata::mongo::repotree::RepoTreeModel;
use git2::{Branch, BranchType, Reference, Repository};

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
    pub fn tree(&self, refs: &Reference, commit_ids: Option<String>) -> anyhow::Result<RepoTree>{
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
    pub fn trees(&self, refs: Branch, owner: String, repo: String) -> anyhow::Result<Vec<RepoTreeModel>>{
        let branch_name = refs.name()?.unwrap().to_string();
        let refs = refs.into_reference();
        let tree = refs.peel_to_commit();
        if tree.is_err(){
            return Err(tree.err().unwrap().into());
        }
        let mut commit_id = tree?;
        let mut result = Vec::new();
        loop {
            let tree = GitCommits::build_tree(&self.repo, commit_id.tree()?.id(), "".to_string());
            if tree.is_err(){
                break;
            }
            result.push(RepoTreeModel{
                hash: commit_id.id().to_string(),
                branch: branch_name.clone(),
                owner: owner.clone(),
                tree: tree?,
                repo: repo.clone(),
                time: commit_id.time().seconds()
            });
            match commit_id.parent(0){
                Ok(parent) => commit_id = parent,
                Err(_) => break
            }
        }
        Ok(result)
    }
}

