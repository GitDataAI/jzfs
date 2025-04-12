use git2::{BranchType, DiffOptions};
use serde::{Deserialize, Serialize};
use crate::GitParam;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct GitCommit {
    pub id: String,
    pub branch_name: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub time: i64,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct GitCommitTree {
    commit: GitCommit,
    tree: GitTree,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct GitTreeCommit {
    pub root: String,
    pub name: String,
    pub typ: String, // Blob | Tree
    pub size: usize,
    pub children: Vec<GitTree>,
    pub commit: GitCommit,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct GitTree {
    pub root: String,
    pub name: String,
    pub typ: String, // Blob | Tree
    pub size: usize,
    pub children: Vec<GitTree>,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct ListCommitParma {
    pub branch: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub end_id: Option<String>,
    pub start_id: Option<String>,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct TreeCommitParma {
    pub branch: Option<String>,
    pub id: Option<String>,
}


fn build_tree(root_tree: &mut GitTree, root: Vec<String>, name: String, size: usize) {
    if root.is_empty() {
        root_tree.children.push(GitTree {
            root: root_tree.root.clone(),
            name,
            typ: "Blob".to_string(),
            size,
            children: vec![],
        });
    }else {
        let rs = root.first().unwrap();
        if let Some(tree) = root_tree.children.iter_mut().find(|x| x.name == *rs) {
            build_tree(tree, root[1..].to_vec(), name, size);
        }else {
            root_tree.children.push(GitTree {
                root: root_tree.root.clone(),
                name: rs.clone(),
                typ: "Tree".to_string(),
                size: 0,
                children: vec![],
            });
            build_tree(
                root_tree.children.last_mut().unwrap(),
                root[1..].to_vec(),
                name,
                size,
            );
        }
    }
}


impl GitParam {
    pub fn list_v2_commit(&mut self, parma: ListCommitParma) -> anyhow::Result<Vec<GitCommitTree>> {
        let repo = self.repo()?;
        let refs = match parma.branch {
            Some(branch)=> repo.find_branch(&branch, BranchType::Local)?.into_reference(),
            None=> repo.head()?,
        };
        let refs = refs.resolve()?;
        let mut commit = if let Some(id) = parma.start_id {
            let oid = git2::Oid::from_str(&id)?;
            repo.find_commit(oid)?
        }else {
            refs.peel_to_commit()?
        };
        if let Some(offset) = parma.offset {
            for _ in 0..offset {
                commit = commit.parent(0)?;
            }
        }
        let limit = parma.limit.unwrap_or_else(|| 0);
        let mut commits:Vec<GitCommitTree> = vec![];
        if commit.parent_count() == 0 {
            let cmt = GitCommit {
                id: commit.id().to_string(),
                branch_name: refs.name().unwrap_or("N/A").to_string(),
                message: commit.message().unwrap_or("N/A").to_string(),
                author: commit.author().name().unwrap_or("N/A").to_string(),
                email: commit.author().email().unwrap_or("N/A").to_string(),
                time: commit.time().seconds(),
            };
            let now_tree = commit.tree()?;
            let diff = repo.diff_tree_to_tree(
                None,
                Some(&now_tree),
                Some(&mut DiffOptions::new())
            )?;

            let mut root_tree = GitTree {
                root: "".to_string(),
                name: "".to_string(),
                typ: "Tree".to_string(),
                size: 0,
                children: vec![],
            };
            for deltas in diff.deltas() {
                let new_file = deltas.new_file();
                let path = match new_file.path(){
                    Some(path) => match path.to_str() {
                        Some(path) => path.to_string(),
                        None => continue,
                    },
                    None => continue,
                };
                let size = new_file.size();
                let (root, name) = if let Some((root, name)) = path.rsplit_once("/") {
                    (root.to_string(), name.to_string())
                }else {
                    ("".to_string(),path.as_str().to_string())
                };
                let root = root.split("/").map(|x|x.to_string()).collect::<Vec<_>>();

                build_tree(&mut root_tree, root, name, size as usize);
            }

            commits.push(GitCommitTree {
                commit: cmt,
                tree: root_tree,
            });
            return Ok(commits);
        }

        loop {
            if limit != 0 && commits.len() >= limit {
                break;
            }
            let parent = match commit.parent(0){
                Ok(parent) => parent,
                Err(_) => break,
            };
            let now_tree = match commit.tree() {
                Ok(tree) => tree,
                Err(_) => break,
            };
            let pre_tree = match parent.tree(){
                Ok(tree) => tree,
                Err(_) => break,
            };
            let diff = match repo.diff_tree_to_tree(
                Some(&now_tree),
                Some(&pre_tree),
                Some(&mut DiffOptions::new())
            ) {
                Ok(diff) => diff,
                Err(_) => break,
            };
            let cmt = GitCommit {
                id: commit.id().to_string(),
                branch_name: refs.name().unwrap_or("N/A").to_string(),
                message: commit.message().unwrap_or("N/A").to_string(),
                author: commit.author().name().unwrap_or("N/A").to_string(),
                email: commit.author().email().unwrap_or("N/A").to_string(),
                time: commit.time().seconds(),
            };
            let mut root_tree = GitTree {
                root: "".to_string(),
                name: "".to_string(),
                typ: "Tree".to_string(),
                size: 0,
                children: vec![],
            };
            for deltas in diff.deltas() {
                let new_file = deltas.new_file();
                let path = match new_file.path(){
                    Some(path) => match path.to_str() {
                        Some(path) => path.to_string(),
                        None => continue,
                    },
                    None => continue,
                };
                let size = new_file.size();
                let (root, name) = if let Some((root, name)) = path.rsplit_once("/") {
                    (root.to_string(), name.to_string())
                }else {
                    ("".to_string(),path.as_str().to_string())
                };
                let root = root.split("/").map(|x|x.to_string()).collect::<Vec<_>>();

                build_tree(&mut root_tree, root, name, size as usize);
            }
            commits.push(GitCommitTree {
                commit: cmt,
                tree: root_tree,
            });
            commit = parent;

        };
        Ok(commits)
    }
}


impl GitTree {
    pub fn find_path(&self, _path: Vec<&str>) -> anyhow::Result<bool> {
        todo!()
    }
}