#![allow(unused)]


use crate::blob::GitBlob;
use git2::{BranchType, DiffOptions, Tree, TreeWalkResult};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
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
        let head = match self.repository.find_branch(&branches, BranchType::Local){
            Ok(head) => head,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.message())),
        };
        let head = head.into_reference();
        let tree = match head.peel_to_tree() {
            Ok(tree) => tree,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.message())),
        };
        let mut commit = match head.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.message())),
        };
        let mut commits = vec![];
        while let Ok(parent) = commit.parent(0) {
            if let Ok(now) = commit.tree() {
                if let Ok(pre) = parent.tree() {
                    if let Ok(diff) = self.repository.diff_tree_to_tree(
                        Some(&tree),
                        Some(&parent.tree().unwrap()),
                        Some(&mut DiffOptions::new())
                    ){
                        let mut deltas = vec![];
                        // diff.foreach(&mut |delta, _ | {
                        //     deltas.push(PathBuf::from(delta.new_file().path().unwrap_or(PathBuf::new().as_path())));
                        //     true
                        // },None, None,None);
                        for delta in diff.deltas() {
                            deltas.push(PathBuf::from(delta.new_file().path().unwrap_or(PathBuf::new().as_path())));
                        }
                        commits.push((Commit {
                            id: commit.id().to_string(),
                            msg: commit.message().unwrap_or("").to_string(),
                            time: chrono::DateTime::from_timestamp(commit.time().seconds(),0).unwrap().timestamp().to_string(),
                            author: commit.author().name().unwrap_or("").to_string(),
                            email: commit.author().email().unwrap_or("").to_string(),
                        },deltas));
                        
                    }
                }
            }
            commit = parent;
        }
        let mut rootless = GitTree {
            id: "".to_string(),
            dir: "".to_string(),
            name: "".to_string(),
            child: vec![],
            is_dir: true,
            commit: vec![],
        };
        
        let _ = tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            let mut paths = root.split('/').collect::<Vec<_>>();
            paths.push(entry.name().unwrap_or("/"));
            paths = paths
                .iter()
                .filter(|x|!x.is_empty()).copied()
                .collect::<Vec<_>>();
            let name = entry.name().unwrap_or("/").to_string();
            let id = entry.id().to_string();
            let mut cs: Vec<Commit> = vec![];
            let ph = PathBuf::new().join(root).join(name.clone());
            for (cmt,diff) in commits.iter() {
                for i in diff{
                    if i.eq(ph.as_path()) {
                        cs.push(cmt.clone());
                    }
                    if !cs.is_empty() {
                        break;
                    }
                }
                
            }
            match entry.kind() {
                Some(git2::ObjectType::Tree) => {
                    if let Ok(x) = rootless.tree_walk(paths, name, id, true,cs) {
                        rootless = x;
                    }
                }
                Some(git2::ObjectType::Blob) => {
                    if let Ok(x) = rootless.tree_walk(paths, name, id, false,cs) {
                        rootless = x;
                    }
                }
                _=>{}
            };
            TreeWalkResult::Ok
        });
        Ok(rootless)
    }
}


impl GitTree {
    pub fn tree_walk(&mut self, path: Vec<&str>, name: String, id: String, is_dir: bool, commit: Vec<Commit>) -> io::Result<GitTree> {
        if path.len() == 1 {
            self.child.push(Self {
                id,
                dir: path.join("/"),
                name,
                child: vec![],
                is_dir,
                commit
            });
            Ok(self.clone())
        } else {
            let mut path = path.clone();
            let dir = path.remove(0);
            let mut child = self.child.clone();
            for child in child.iter_mut() {
                if child.dir == dir {
                    child.tree_walk(path.clone(), name.clone(), id.clone(), is_dir,commit.clone())?;
                }
            }
            self.child = child;
            Ok(self.clone())
        }
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
        dbg!(res);
        Ok(())
    }
}