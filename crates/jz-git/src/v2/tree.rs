use git2::{ObjectType, TreeWalkResult};
use serde::{Deserialize, Serialize};
use crate::GitParam;
use crate::tree::GitTreeParam;

#[derive(Clone,Deserialize,Serialize,Debug)]
pub struct GitTreeV2 {
    pub path: String,
    pub name: String,
    pub sha1: String,
    pub blob: Vec<GitBlobV2>,
    pub tree: Vec<GitTreeV2>,
}

#[derive(Clone,Deserialize,Serialize,Debug)]
pub struct GitBlobV2 {
    pub name: String,
    pub sha1: String,
    pub size: i64,
}


impl GitParam {
    pub fn tree_v2(&mut self, param: GitTreeParam) -> anyhow::Result<GitTreeV2> {
        let repo = self.repo()?;
        let refer = match param.branches {
            Some(ref branches) => {
                repo.find_branch(branches, git2::BranchType::Local)?.into_reference()
            }
            None => {
                repo.head()?
            }
        };
        let commit = match param.sha {
            Some(ref sha) => {
                repo.revparse_single(sha)?.peel_to_commit()?
            }
            None => {
                refer.peel_to_commit()?
            }
        };
        let tree = commit.tree()?;
        let mut path = param.path.clone();
        if path == "/" {
            path = "".to_string();
        }else if !path.ends_with("/") && !path.is_empty() { 
            path += "/";
        }
        let name = param.path.split("/").last().unwrap_or("").to_string();
        let mut roots = GitTreeV2 {
            path: path.clone(),
            name: name.clone(),
            sha1: tree.id().to_string(),
            blob: vec![],
            tree: vec![],
        };
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            if root != path {
                return TreeWalkResult::Ok;
            }
            match entry.kind() {
                None => {
                    return TreeWalkResult::Ok;
                }
                Some(kind) => {
                    match kind {
                        ObjectType::Blob => {
                            let size = match repo.find_object(entry.id(), Some(ObjectType::Blob)) {
                                Ok(blob) => {
                                    blob.as_blob().map(|blob| blob.size()).unwrap_or(0)
                                }
                                Err(_) => {
                                    0
                                }
                            };
                            let blob = GitBlobV2 {
                                size: size as i64,
                                name: entry.name().unwrap_or("").to_string(),
                                sha1: entry.id().to_string(),
                            };
                            roots.blob.push(blob);
                        }
                        ObjectType::Tree => {
                            let tree = GitTreeV2 {
                                path: root.to_string(),
                                name: entry.name().unwrap_or("").to_string(),
                                sha1: entry.id().to_string(),
                                blob: vec![],
                                tree: vec![],
                            };
                            roots.tree.push(tree);
                        }
                        _ => {
                            return TreeWalkResult::Ok
                        }
                    }
                }
            }
            TreeWalkResult::Ok
        }).ok();
        Ok(roots)
    }
}

#[test]
fn test_tree_v2() {
    use std::path::PathBuf;
    let mut git = GitParam::new(PathBuf::from("/home/zhenyi/文档"), "r-nacos".to_string()).unwrap();
    let res = git.tree_v2(GitTreeParam {
        path: "".to_string(),
        branches: Option::from("pr_ts".to_string()),
        sha: None,
    });
    dbg!(res);
    
}