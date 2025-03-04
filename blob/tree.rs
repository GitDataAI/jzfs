#![allow(unused)]

use crate::blob::GitBlob;
use git2::{BranchType, DiffOptions, Tree, TreeWalkResult};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};
use crate::blob::blob::Commit;

#[derive(Clone,Debug,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub struct GitTree {
    pub id: String,
    pub dir: String,
    pub name: String,
    pub child: Vec<GitTree>,
    pub is_dir: bool,
    pub commit: Vec<Commit>,
}

impl GitBlob {
    pub fn tree(&self, branches: String) -> io::Result<GitTree> {
        let head = self.repository.find_branch(&branches, BranchType::Local)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
        let head_ref = head.into_reference();
        let tree = head_ref.peel_to_tree()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
        let mut commit = head_ref.peel_to_commit()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;

        let mut path_commits = std::collections::HashMap::new();
        let mut limit = 0;
        while limit < 20000 {
            let parent = match commit.parent(0) {
                Ok(p) => p,
                Err(_) => break,
            };

            let now_tree = commit.tree().map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
            let pre_tree = parent.tree().map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;

            let diff = self.repository.diff_tree_to_tree(
                Some(&now_tree),
                Some(&pre_tree),
                Some(&mut DiffOptions::new())
            ).map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;

            let cmt = Commit {
                id: commit.id().to_string(),
                msg: commit.message().unwrap_or("").to_string(),
                time: chrono::DateTime::from_timestamp(commit.time().seconds(),0)
                    .unwrap().timestamp().to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
            };

            for delta in diff.deltas() {
                if let Some(path) = delta.new_file().path() {
                    let path = Path::new("/").join(path);
                    path_commits.entry(path)
                        .or_insert_with(|| cmt.clone());
                }
            }

            commit = parent;
            limit += 1;
        }

        let mut root_tree = GitTree {
            id: String::new(),
            dir: String::new(),
            name: String::new(),
            child: vec![],
            is_dir: true,
            commit: vec![],
        };

        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            let name = entry.name().unwrap_or_default().to_string();
            let path = Path::new(root).join(&name);

            let normalized_path = Path::new("/").join(
                path.strip_prefix("/").unwrap_or(&path)
            );

            let commit = path_commits.get(&normalized_path)
                .cloned()
                .into_iter()
                .collect();

            let components = normalized_path.iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let node = GitTree {
                id: entry.id().to_string(),
                dir: components[..components.len()-1].iter()
                    .map(|s| s.to_str().unwrap_or_default())
                    .collect::<Vec<_>>()
                    .join("/"),
                name: components.last()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string(),
                child: vec![],
                is_dir: matches!(entry.kind(), Some(git2::ObjectType::Tree)),
                commit,
            };

            let mut current = &mut root_tree;
            for part in components.iter().take(components.len().saturating_sub(1)) {
                let part_str = part.to_str().unwrap_or_default();
                if !current.child.iter().any(|c| c.name == part_str) {
                    current.child.push(GitTree {
                        id: String::new(),
                        dir: current.dir.clone(),
                        name: part_str.to_string(),
                        child: vec![],
                        is_dir: true,
                        commit: vec![],
                    });
                }
                current = current.child
                    .iter_mut()
                    .find(|c| c.name == part_str)
                    .unwrap();
            }

            if let Some(existing) = current.child.iter_mut()
                .find(|c| c.name == node.name)
            {
                *existing = node;
            } else {
                current.child.push(node);
            }

            TreeWalkResult::Ok
        }).ok();

        Ok(root_tree.child.first().unwrap_or(&root_tree).clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::blob::GitBlob;
    use std::io;

    #[test]
    fn test_tree() -> io::Result<()> {
        let blob = GitBlob::new("/home/zhenyi/文档/gitdata".into()).unwrap();
        let branch = blob.repository.find_branch("main", git2::BranchType::Local).unwrap();
        
        let res = blob.tree("main".to_string())?;
        dbg!(res.child.len());
        Ok(())
    }
}