use crate::api::dto::repo_dto::RepoTree;
use crate::git::dtos::FileDto;
use crate::git::tree::GitTree;
use git2::{Branch, Commit, Repository};

pub struct GitCommits<'a>{
    branch: Branch<'a>,
}

impl <'a>GitCommits<'a>{
    pub fn new(branch: Branch<'a>) -> Self{
        Self{
            branch
        }
    }
    pub fn commits(&self) -> anyhow::Result<Vec<Commit>>{
        let mut result = Vec::new();
        let head = self.branch.get().peel_to_commit();
        if head.is_err(){
            return Err(head.err().unwrap().into());
        }        
        let mut commit = head?;
        loop {
            result.push(commit.clone());
            match commit.parent(0){
                Ok(parent) => commit = parent,
                Err(_) => break
            }
        }
        Ok(result)
    }
    pub fn tree(self) -> anyhow::Result<Vec<FileDto>>{
        let tree = self.branch.into_reference().peel_to_commit();
        if tree.is_err(){
            return Err(tree.err().unwrap().into());
        }
        let tree = tree?;
        let cmt = GitTree::new(tree);
        let map = cmt.tree()?;
        Ok(map)
    }
    pub fn build_tree(repo: &Repository, tree_id: git2::Oid, base_path: String) -> anyhow::Result<RepoTree> {
        let tree = repo.find_tree(tree_id)?;
        let mut children = Vec::new();
        for entry in tree.iter() {
            if let Some(name) = entry.name() {
                let full_path = if base_path.is_empty() {
                    name.to_string()
                } else {
                    format!("{}/{}", base_path, name)
                };
                let is_dir = entry.kind() == Some(git2::ObjectType::Tree);
                if is_dir {
                    if let Some(entry_id) = entry.id().to_string().parse::<git2::Oid>().ok() {
                        if let Ok(child_tree) = Self::build_tree(repo, entry_id, full_path.clone()) {
                            children.push(child_tree);
                        }
                    }
                } else {
                    children.push(RepoTree {
                        name: name.to_string(),
                        is_dir,
                        path: full_path.clone(),
                        children: Vec::new(),
                    });
                }
            }
        }
        Ok(RepoTree {
            name: base_path.clone().to_string().trim_end_matches('/')
                .rsplit('/')
                .next().unwrap_or("").to_string(),
            is_dir: true,
            path: base_path,
            children,
        })
    }
}

