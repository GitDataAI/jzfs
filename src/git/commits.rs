use git2::{Branch, Commit};
use crate::git::dtos::FileDto;
use crate::git::tree::GitTree;

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
}