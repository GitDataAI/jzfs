use crate::GitContext;
use error::AppError;
use git2::{ObjectType, TreeWalkResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TreeParam {
    pub refs: Option<String>,
    pub tree_oid: Option<String>,
    pub dir: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TreeItem {
    pub path: String,
    pub kind: TreeKind,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub enum TreeKind {
    Tree,
    Blob,
}

#[derive(Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TreeItemLastCommit {
    pub item: TreeItem,
    pub commit_oid: String,
    pub commit_message: String,
    pub commit_time: i64,
    pub commit_offset: i64,
}

impl GitContext {
    pub fn tree(&self, param: TreeParam) -> Result<Vec<TreeItem>, AppError> {
        let repo = self.repo()?;
        let refs = match param.refs {
            None => repo.head()?,
            Some(refs) => repo
                .find_branch(&refs, git2::BranchType::Local)?
                .into_reference(),
        };
        let tree = refs.peel_to_tree()?;
        let mut result = HashSet::new();
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            if let Some(name) = entry.name().map(|x| x.to_string())
                && root.starts_with(&format!("{}", param.dir))
            {
                #[allow(irrefutable_let_patterns)]
                if let Ok(path) = PathBuf::from_str(&format!("{}{}", root, name)) {
                    if let Some(parent) = path.parent() {
                        let parent_path = parent
                            .to_str()
                            .unwrap()
                            .to_string()
                            .replace("/", "")
                            .replace("\\", "");
                        let need_dir = param.dir.replace("/", "").replace("\\", "");
                        if parent_path == need_dir {
                            match entry.kind() {
                                None => {}
                                Some(kind) => match kind {
                                    ObjectType::Tree => {
                                        result.insert(TreeItem {
                                            path: format!("{}", root),
                                            kind: TreeKind::Tree,
                                            name,
                                        });
                                    }
                                    ObjectType::Blob => {
                                        result.insert(TreeItem {
                                            path: format!("{}", root),
                                            kind: TreeKind::Blob,
                                            name,
                                        });
                                    }
                                    _ => {}
                                },
                            }
                        }
                    }
                }
            }
            TreeWalkResult::Ok
        })
        .ok();
        Ok(result.iter().map(|x| x.clone()).collect::<Vec<_>>())
    }
    pub fn tree_item_last_commit(
        &self,
        param: Vec<TreeItem>,
    ) -> Result<Vec<TreeItemLastCommit>, AppError> {
        let repo = self.repo()?;
        let mut result = Vec::new();

        for item in param {
            // Construct the full path
            let item_path = if item.path.ends_with('/') {
                format!("{}{}", item.path, item.name)
            } else if item.path == "" {
                item.name.clone()
            } else {
                format!("{}/{}", item.path, item.name)
            };
            let mut revwalk = repo.revwalk()?;
            revwalk.push_head()?;
            revwalk.set_sorting(git2::Sort::TIME)?;
            let mut found_commit = false;
            for oid in revwalk {
                let oid = oid?;
                let commit = repo.find_commit(oid)?;
                let tree = commit.tree()?;
                if let Ok(_) = tree.get_path(std::path::Path::new(&item_path)) {
                    if commit.parent_count() > 0 {
                        let parent = commit.parent(0)?;
                        let parent_tree = parent.tree()?;

                        let mut diff_opts = git2::DiffOptions::new();
                        diff_opts.pathspec(&item_path);

                        let diff = repo.diff_tree_to_tree(
                            Some(&parent_tree),
                            Some(&tree),
                            Some(&mut diff_opts),
                        )?;

                        if diff.deltas().count() > 0 {
                            result.push(TreeItemLastCommit {
                                item: item.clone(),
                                commit_oid: commit.id().to_string(),
                                commit_message: commit.message().unwrap_or("").to_string(),
                                commit_time: commit.time().seconds(),
                                commit_offset: commit.time().offset_minutes() as i64,
                            });
                            found_commit = true;
                            break;
                        }
                    } else {
                        result.push(TreeItemLastCommit {
                            item: item.clone(),
                            commit_oid: commit.id().to_string(),
                            commit_message: commit.message().unwrap_or("").to_string(),
                            commit_time: commit.time().seconds(),
                            commit_offset: commit.time().offset_minutes() as i64,
                        });
                        found_commit = true;
                        break;
                    }
                }
            }

            if !found_commit {
                result.push(TreeItemLastCommit {
                    item,
                    commit_oid: String::new(),
                    commit_message: String::new(),
                    commit_time: 0,
                    commit_offset: 0,
                });
            }
        }

        Ok(result)
    }
}

#[test]
fn test_tree() {
    let ctx = GitContext {
        path_dir: PathBuf::from("E:\\Code\\acl-anthology.git"),
    };
    let res = ctx
        .tree(TreeParam {
            refs: None,
            tree_oid: Some("75010f1153f8f7237022b277f37a44aeef1afee0".to_string()),
            dir: "".to_string(),
        })
        .unwrap();
    println!("len: {:?}", res.len());
    let res = ctx.tree_item_last_commit(res).unwrap();
    dbg!(res);
}
