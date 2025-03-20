use crate::commits::GitCommit;
use crate::tree::TreeEntityItem;
use crate::GitParam;
use git2::{BranchType, Commit, DiffOptions, TreeWalkResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Hash,Deserialize)]
pub struct GitCommitTree {
    pub commit: String,
    pub author: String,
    pub email: String,
    pub date: i64,
    pub msg: String,
    pub path: String,
    pub name: String,
    pub r#type: String,
}

impl PartialEq<Self> for GitCommitTree {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.name == other.name && self.r#type == other.r#type
    }
}



impl Eq for GitCommitTree {}

impl GitParam {
    // pub fn tree_dir(
    //     &mut self,
    //     branch: Option<String>,
    //     start_commit: Option<String>,
    // ) -> anyhow::Result<Vec<GitCommitTree>> {
    //     let repo = match self.repo() {
    //         Ok(repo) => repo,
    //         Err(_) => return Err(anyhow::anyhow!("repo error")),
    //     };
    //     let head = match branch {
    //         Some(branch) => match repo.find_branch(branch.as_str(), BranchType::Local) {
    //             Ok(head) => head.into_reference(),
    //             Err(_) => return Err(anyhow::anyhow!("branch error")),
    //         },
    //         None => match repo.head() {
    //             Ok(head) => head,
    //             Err(_) => return Err(anyhow::anyhow!("head error")),
    //         },
    //     };
    //     let mut commit = match start_commit {
    //         Some(commit) => match repo.find_commit(Oid::from_str(commit.as_str())?) {
    //             Ok(commit) => commit,
    //             Err(_) => return Err(anyhow::anyhow!("commit error")),
    //         },
    //         None => match head.peel_to_commit() {
    //             Ok(commit) => commit,
    //             Err(_) => return Err(anyhow::anyhow!("commit error")),
    //         },
    //     };
    //
    //     let (blob_paths, tree_paths): (HashSet<_>, HashSet<_>) = param.into_iter().fold(
    //         (HashSet::new(), HashSet::new()),
    //         |(mut blobs, mut trees), item| {
    //             if item.r#type == "blob" {
    //                 blobs.insert(item.path);
    //             } else {
    //                 trees.insert(item.path);
    //             }
    //             (blobs, trees)
    //         },
    //     );
    //     let mut results = HashSet::new();
    //     loop {
    //         if results.len() == blob_paths.len() + tree_paths.len() {
    //             break;
    //         }
    //         let parent = match commit.parent(0) {
    //             Ok(p) => p,
    //             Err(_) => break,
    //         };
    //
    //         let (new_tree, pre_tree) = match (commit.tree(), parent.tree()) {
    //             (Ok(new), Ok(old)) => (new, old),
    //             _ => break,
    //         };
    //
    //         let diff = repo
    //             .diff_tree_to_tree(Some(&pre_tree), Some(&new_tree), None)
    //             .map_err(|e| anyhow!("diff failed: {}", e))?;
    //
    //         let paths: Vec<String> = diff
    //             .deltas()
    //             .filter_map(|d| d.new_file().path())
    //             .filter_map(|p| p.to_str().map(String::from))
    //             .collect();
    //
    //         let has_relevant_change = paths.iter().any(|p| {
    //             blob_paths.contains(p)
    //                 || tree_paths.iter().any(|t| {
    //                     p.starts_with(t) && p.split('/').count() == t.split('/').count() + 1
    //                 })
    //         });
    //         if !has_relevant_change {
    //             commit = parent;
    //             continue;
    //         }
    //
    //         for path in paths.iter().filter(|p| blob_paths.contains(*p)) {
    //             let (dir_path, name) = split_path(path);
    //             let entry = create_entry(&commit, dir_path, name, "blob");
    //             results.insert(entry);
    //         }
    //
    //         for path in &paths {
    //             for t in &tree_paths {
    //                 if path.starts_with(t) && path.split('/').count() == t.split('/').count() + 1 {
    //                     let (dir_path, name) = split_path(t); // 提取父目录名
    //                     let entry = create_entry(&commit, dir_path, name, "tree");
    //                     results.insert(entry);
    //                 }
    //             }
    //         }
    //         commit = parent;
    //     }
    //     let results: Vec<_> = results.into_iter().collect();
    //     let mut result = vec![];
    //     for item in results {
    //         if result
    //             .iter()
    //             .find(|x: &&GitCommitTree| {
    //                 x.path == item.path && x.name == item.name && x.r#type == item.r#type
    //             })
    //             .is_none()
    //         {
    //             result.push(item);
    //         }
    //     }
    //     Ok(result)
    // }
    pub fn tree_msg(&mut self, branch: Option<String>) -> anyhow::Result<Vec<GitCommitTree>> {
        if branch.is_none() {
            return Err(anyhow::anyhow!("branch error"));
        }
        let repo = self.repo()?;
        let branch_name = branch.unwrap_or_else(|| "master".to_string());
        let head = repo.find_branch(&branch_name, BranchType::Local)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;

        let head_ref = head.into_reference();
        let tree = head_ref.peel_to_tree()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
        let mut commit = head_ref.peel_to_commit()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
        let mut path_commits = std::collections::HashMap::new();

        loop {
            let parent = match commit.parent(0) {
                Ok(p) => p,
                Err(_) => break,
            };
            let now_tree = commit.tree().map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
            let pre_tree = parent.tree().map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;

            let diff = repo.diff_tree_to_tree(
                Some(&now_tree),
                Some(&pre_tree),
                Some(&mut DiffOptions::new())
            ).map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;

            let cmt = GitCommit {
                id: commit.id().to_string(),
                msg: commit.message().unwrap_or("N/A").to_string(),
                date: commit.time().seconds(),
                author: commit.author().name().unwrap_or("N/A").to_string(),
                email: commit.author().email().unwrap_or("N/A").to_string(),
            };

            for delta in diff.deltas() {
                if let Some(path) = delta.new_file().path() {
                    let path_str = path.to_str().unwrap_or_default().to_string();

                    // 更新文件路径的提交
                    path_commits.entry(path_str.clone())
                        .and_modify(|existing: &mut GitCommit| {
                            if cmt.date > existing.date {
                                *existing = cmt.clone();
                            }
                        })
                        .or_insert_with(|| cmt.clone());

                    // 更新所有父目录路径的提交
                    let mut current_path = Path::new(&path_str);
                    while let Some(parent_path) = current_path.parent() {
                        let parent_str = parent_path.to_str().unwrap_or_default().to_string();
                        if !parent_str.is_empty() {
                            path_commits.entry(parent_str.clone())
                                .and_modify(|existing| {
                                    if cmt.date > existing.date {
                                        *existing = cmt.clone();
                                    }
                                })
                                .or_insert_with(|| cmt.clone());
                        }
                        current_path = parent_path;
                    }
                }
            }
            commit = parent;
        }
        let mut root_tree = vec![];
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            let name = entry.name().unwrap_or_default().to_string();
            let path = Path::new(root).join(&name).to_str().unwrap_or_default().to_string();
            match entry.kind() {
                Some(git2::ObjectType::Blob) => {
                    let commit = path_commits.get(&path)
                        .cloned()
                        .unwrap_or_else(|| GitCommit {
                            id: "".to_string(),
                            msg: "N/A".to_string(),
                            date: 0,
                            author: "N/A".to_string(),
                            email: "".to_string(),
                        });
                    root_tree.push(GitCommitTree {
                        commit: commit.id,
                        author: commit.author,
                        email: commit.email,
                        date: commit.date,
                        msg: commit.msg,
                        path,
                        name,
                        r#type: "blob".to_string(),
                    });
                }
                Some(git2::ObjectType::Tree) => {
                    let commit = path_commits.get(&path)
                        .cloned()
                        .unwrap_or_else(|| GitCommit {
                            id: "".to_string(),
                            msg: "N/A".to_string(),
                            date: 0,
                            author: "N/A".to_string(),
                            email: "".to_string(),
                        });
                    root_tree.push(GitCommitTree {
                        commit: commit.id,
                        author: commit.author,
                        email: commit.email,
                        date: commit.date,
                        msg: commit.msg,
                        path,
                        name,
                        r#type: "tree".to_string(),
                    });
                }
                _ => {}
            }
            TreeWalkResult::Ok
        }).ok();

        Ok(root_tree)
    }
    pub fn tree_dir_revwalk(
        &mut self,
        param: Vec<TreeEntityItem>,
    ) -> anyhow::Result<Vec<GitCommitTree>> {
        if param.is_empty() {
            return Ok(vec![]);
        }
        let repo = match self.repo() {
            Ok(repo) => repo,
            Err(_) => return Err(anyhow::anyhow!("repo error")),
        };
        let (blob_paths, tree_paths): (HashSet<_>, HashSet<_>) = param.into_iter().fold(
            (HashSet::new(), HashSet::new()),
            |(mut blobs, mut trees), item| {
                if item.r#type == "blob" {
                    blobs.insert(item.path);
                } else {
                    trees.insert(item.path);
                }
                (blobs, trees)
            },
        );

        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        revwalk.simplify_first_parent()?;
        revwalk.push_head()?;
        let mut results: HashSet<GitCommitTree> = HashSet::new();

        let mut diff_option = DiffOptions::default();
        while let Some(Ok(item)) = revwalk.next() {
            if results.len() == blob_paths.len() + tree_paths.len() {
                break;
            }
            let commit = match repo.find_commit(item) {
                Ok(commit) => commit,
                Err(_) => continue,
            };
            if commit.parent_count() >= 1 {
                let prev_commit = match commit.parent(0) {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                let tree = match commit.tree() {
                    Ok(tree) => tree,
                    Err(_) => continue,
                };
                let prev_tree = match prev_commit.tree() {
                    Ok(tree) => tree,
                    Err(_) => continue,
                };
                let diff = match repo.diff_tree_to_tree(
                    Some(&prev_tree),
                    Some(&tree),
                    Some(&mut diff_option),
                ) {
                    Ok(diff) => diff,
                    Err(_) => continue,
                };

                let paths: Vec<String> = diff
                    .deltas()
                    .filter_map(|d| d.new_file().path())
                    .filter_map(|p| p.to_str().map(String::from))
                    .collect();

                let has_relevant_change = paths.iter().any(|p| {
                    blob_paths.contains(p)
                        || tree_paths.iter().any(|t| {
                            p.starts_with(t) && p.split('/').count() == t.split('/').count() + 1
                        })
                });
                if !has_relevant_change {
                    continue;
                }

                for path in paths.iter().filter(|p| blob_paths.contains(*p)) {
                    let (dir_path, name) = split_path(path);
                    let entry = create_entry(&commit, dir_path, name, "blob");
                    results.insert(entry);
                }

                for path in &paths {
                    for t in &tree_paths {
                        if path.starts_with(t)
                            && path.split('/').count() == t.split('/').count() + 1
                        {
                            let (dir_path, name) = split_path(t);
                            let entry = create_entry(&commit, dir_path, name, "tree");
                            results.insert(entry);
                        }
                    }
                }
            }
        }
        let results: Vec<_> = results.into_iter().collect();
        let mut result = vec![];
        for item in results {
            if result
                .iter()
                .find(|x: &&GitCommitTree| {
                    x.path == item.path && x.name == item.name && x.r#type == item.r#type
                })
                .is_none()
            {
                result.push(item);
            }
        }
        Ok(result)
    }
}

fn split_path(path: &str) -> (String, String) {
    let path = Path::new(path);
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    let parent = path
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    (parent, name)
}

fn create_entry(commit: &Commit, path: String, name: String, type_: &str) -> GitCommitTree {
    GitCommitTree {
        commit: commit.id().to_string(),
        author: commit
            .author()
            .name()
            .map(|s| s.to_string())
            .unwrap_or_default(),
        email: commit
            .author()
            .email()
            .map(|s| s.to_string())
            .unwrap_or_default(),
        date: commit.time().seconds(),
        msg: commit.message().map(|s| s.to_string()).unwrap_or_default(),
        path,
        name,
        r#type: type_.to_string(),
    }
}
