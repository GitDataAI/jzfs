use std::io;
use git2::{Error, Oid, Repository, Tree, TreeWalkMode};
use crate::blob::GitBlob;

impl GitBlob {
    pub fn size(&self, hash: String) -> io::Result<i32> {
        let oid = match Oid::from_str(&hash) {
            Ok(oid) => oid,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid hash")),
        };
        let commit = match self.repository.find_commit(oid) {
            Ok(commit) => commit,
            Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, "Commit not found")),
        };
        let tree = match commit.tree() {
            Ok(tree) => tree,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to get commit tree")),
        };
        
        let size = match calculate_tree_size(&self.repository, tree) {
            Ok(size) => size,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to calculate tree size")),
        };
        
        Ok(size)
    }
}

fn calculate_tree_size(repo: &Repository, tree: Tree) -> Result<i32, Error> {
    let mut size = 0;
    let mut stack = vec![tree];

    while let Some(current_tree) = stack.pop() {
        current_tree.walk(TreeWalkMode::PreOrder, |root, entry| {
            // let path = format!("{}{}", root, entry.name().unwrap_or(""));
            let obj = match repo.find_object(entry.id(), None) {
                Ok(obj) => obj,
                Err(_) => return git2::TreeWalkResult::Skip,
            };
            match obj.kind() {
                Some(git2::ObjectType::Blob) => {
                    size += obj.into_blob().unwrap().size()
                }
                Some(git2::ObjectType::Tree) => {
                    let subtree =match repo.find_tree(entry.id()) {
                        Ok(subtree) => subtree,
                        Err(_) => return git2::TreeWalkResult::Skip,
                    };
                    stack.push(subtree);
                }
                _ => {}
            }
            git2::TreeWalkResult::Ok
        })?;
    }
    Ok(size as i32)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_calculate_tree_size() {
        let repo = GitBlob::new(PathBuf::from_str("/home/zhenyi/文档/GitDataAI/GitDataWeb").unwrap()).unwrap();
        let size = repo.size("de697cb9571b49217919704de0666d0386ae45e5".to_string());
        dbg!(size.unwrap() / 1024);
    }
}