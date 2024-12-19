use git2::Repository;
use libs::git::branchs::GitBranch;
use libs::metadata::model::repo::{repo_branch, repo_commit};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let repo = Repository::open("E:/gittp")?;
    let branchs = GitBranch::new(repo);
    let branchs = branchs.branchs();
    if branchs.is_err() {
        return Err(branchs.err().unwrap());
    }
    let repo_uid = Uuid::new_v4();
    let mut map = HashMap::new();
    let branchs = branchs?;
    for branchs_idx in branchs {
        let branch = branchs_idx.name()?.unwrap().to_string();
        let refs = branchs_idx.into_reference();
        let peel_commit = refs.peel_to_commit();
        let mut commits = Vec::new();
        let branch_uid = Uuid::new_v4();
        if let Ok(commit) = peel_commit {
            let mut current_commit = Some(commit);
            while let Some(commit) = current_commit {
                let commit_id = commit.id().to_string();
                println!("commit_id: {}", commit_id);
                let commit_time = OffsetDateTime::from_unix_timestamp(commit.time().seconds())?;
                let commit_bio = commit.message().unwrap_or("").to_string();
                let (commit_username, commit_email) = {
                    if commit.author().name().is_some() {
                        (
                            commit.author().name().unwrap_or("").to_string(),
                            commit.author().email().unwrap_or("").to_string(),
                        )
                    } else {
                        ("".to_string(), "".to_string())
                    }
                };
                let commit_model = repo_commit::Model {
                    uid: Uuid::new_v4(),
                    repo_id: repo_uid,
                    branch_id: branch_uid,
                    bio: commit_bio,
                    commit_user: commit_username,
                    commit_email: commit_email,
                    commit_id: commit_id,
                    created_at: commit_time,
                };
                commits.push(commit_model);

                // 移动到下一个父提交
                current_commit = commit.parent(0).ok();
            }
        } else {
            continue;
        }
        let branch_model = repo_branch::Model {
            uid: branch_uid,
            repo_id: repo_uid,
            branch: branch,
            head: commits.first().map(|x| x.uid.clone()),
            visible: true,
            protect: false,
            created_at: commits.last().unwrap().created_at,
            updated_at: commits.first().unwrap().created_at,
            created_by: Uuid::nil(),
        };
        map.insert(branch_model, commits);
    }
    dbg!(map);
    Ok(())
}
