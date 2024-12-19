use git2::{Repository, TreeWalkMode, TreeWalkResult};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, format_description};

#[derive(Debug,Serialize,Deserialize)]
pub struct RepoTree {
    pub name: String,
    pub is_dir: bool,
    pub path: String,
    pub children: Vec<RepoTree>,
}

fn build_tree(repo: &Repository, tree_id: git2::Oid, base_path: String) -> Result<RepoTree> {
    let tree = repo.find_tree(tree_id)?;
    let mut children = Vec::new();
    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        if let Some(name) = entry.name() {
            let full_path = format!("{}{}", base_path, name);
            if let Some(entry_id) = entry.id().to_string().parse::<git2::Oid>().ok() {
                let is_dir = entry.kind() == Some(git2::ObjectType::Tree);
                if is_dir {
                    if let Ok(child_tree) = build_tree(repo, entry_id, format!("{}/", full_path)) {
                        children.push(child_tree);
                    }
                } else {
                  
                    children.push(RepoTree {
                        name: name.trim_end_matches('/')
                            .rsplit('/')
                            .next().unwrap_or("").to_string(),
                        is_dir,
                        path: full_path.to_string(),
                        children: Vec::new(),
                    });
                }
            }
        }
        TreeWalkResult::Ok
    })?;

    Ok(RepoTree {
        name: base_path.clone().to_string().trim_end_matches('/')
        .rsplit('/')
        .next().unwrap_or("").to_string(),
        is_dir: true,
        path: base_path.clone(),
        children,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let repo = Repository::open("E:/gittp")?;
    let branches = repo.branches(None)?;
    let mut trees = Vec::new();
    for branch in branches {
        let (branch, _) = branch?;
        let branch_name = branch.name()?.unwrap_or("Unnamed").to_string();

        // 获取分支引用的最新提交
        if let Some(reference) = branch.get().peel_to_commit().ok() {
            let commit_id = reference.id().to_string();
            let commit_msg = reference.message().unwrap_or("").to_string();
            let commit_time = reference.time().seconds();
            let commit_time = OffsetDateTime::from_unix_timestamp(commit_time)
                .map(|t| {
                    let format = format_description::well_known::Rfc3339;
                    t.format(&format).unwrap_or("Invalid Time".to_string())
                })
                .unwrap_or("Invalid Time".to_string());
            // 获取提交对应的树
            if let Ok(tree) = reference.tree() {
                let mut repo_tree = build_tree(&repo, tree.id(), "".to_string())?;
                trees.push(repo_tree);
            }
        }
    }
    let json = serde_json::to_string_pretty(&trees)?;
    println!("{json}");
    Ok(())
}
