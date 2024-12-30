use crate::git::git::options::{BlobTree, Commits};
use crate::git::git::GitLocal;
use git2::{DiffOptions, Oid, Reference, Tree};
use std::path::Path;

impl GitLocal {
    fn build_diff_blob_tree(
        &self,
        tree: &Tree,
        parent_tree: Option<&Tree>,
    ) -> Result<BlobTree, git2::Error> {
        let mut diff_options = DiffOptions::new();
        let diff =
            self.repository
                .diff_tree_to_tree(parent_tree, Some(tree), Some(&mut diff_options))?;
        let mut children = Vec::new();
        diff.foreach(
            &mut |delta, _| {
                if let Some(name) = delta.new_file().path() {
                    let size = delta.new_file().size() as usize;
                    children.push(BlobTree {
                        name: name
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                        path: name.to_string_lossy().to_string(),
                        is_dir: false,
                        size,
                        children: Vec::new(),
                    });
                }
                true
            },
            None,
            None,
            None,
        )?;
        Ok(BlobTree {
            name: Path::new("").to_string_lossy().to_string(),
            path: "".to_string(),
            is_dir: true,
            size: 0,
            children,
        })
    }
    pub fn get_commit_diff(&self, commit_hash: &str) -> Result<Commits, git2::Error> {
        let oid = Oid::from_str(commit_hash)?;
        let commit = self.repository.find_commit(oid)?;

        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let branches = self
            .repository
            .branches(None)?
            .filter_map(|branch| {
                branch.ok().and_then(|(branch, _)| {
                    branch.get().target().and_then(|target| {
                        if target == oid {
                            branch.name().ok().flatten().map(String::from)
                        } else {
                            None
                        }
                    })
                })
            })
            .collect::<Vec<_>>()
            .join(",");

        // 构造差异的文件树
        let tree_diff = Self::build_diff_blob_tree(&self, &tree, parent_tree.as_ref())?;
        // 构造 Commits 结构体
        let commit_info = Commits {
            hash_oid: commit.id().to_string(),
            msg: commit.message().unwrap_or("").to_string(),
            username: commit.author().name().unwrap_or("").to_string(),
            email: commit.author().email().unwrap_or("").to_string(),
            branchs: branches,
            time: commit.time().seconds(),
            tree: tree_diff,
        };

        Ok(commit_info)
    }
    pub fn build_tree(&self, tree_id: Oid, base_path: String) -> anyhow::Result<BlobTree> {
        let tree = self.repository.find_tree(tree_id)?;
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
                    if let Some(entry_id) = entry.id().to_string().parse::<Oid>().ok() {
                        if let Ok(child_tree) = Self::build_tree(self, entry_id, full_path.clone())
                        {
                            children.push(child_tree);
                        }
                    }
                } else {
                    let size = if let Some(entry_id) = entry.id().to_string().parse::<Oid>().ok() {
                        if let Ok(child_tree) = self.repository.find_blob(entry_id) {
                            child_tree.content().len()
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                    children.push(BlobTree {
                        name: name.to_string(),
                        is_dir,
                        path: full_path.clone(),
                        children: Vec::new(),
                        size,
                    });
                }
            }
        }
        Ok(BlobTree {
            name: base_path
                .clone()
                .to_string()
                .trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string(),
            is_dir: true,
            path: base_path,
            children,
            size: 0,
        })
    }
    pub fn branch_tree(
        &self,
        refs: &Reference,
        commit_ids: Option<String>,
    ) -> anyhow::Result<BlobTree> {
        let tree = refs.peel_to_commit();
        if tree.is_err() {
            return Err(tree.err().unwrap().into());
        }
        let mut commit_id = tree?;
        let commit = if let Some(id) = commit_ids {
            loop {
                if commit_id.id().to_string() == id {
                    break commit_id;
                } else {
                    match commit_id.parent(0) {
                        Ok(parent) => commit_id = parent,
                        Err(_) => break refs.peel_to_commit()?,
                    }
                }
            }
        } else {
            commit_id
        };
        Self::build_tree(&self, commit.tree()?.id(), "".to_string())
    }
}
