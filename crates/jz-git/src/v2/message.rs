use crate::tree::GitTreeParam;
use crate::GitParam;
use git2::TreeWalkResult;
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct GitMsgV2 {
    pub sha1: String,
    pub name: String,
    pub path: String,
    pub message: String,
    pub cmt_id: String,
    pub author: String,
    pub email: String,
    pub date: i64,
    pub size: u64,
    pub r#type: String,
}


impl GitParam {
    pub fn msg_v2(&mut self, param: GitTreeParam) -> anyhow::Result<Vec<GitMsgV2>> {
        let repo = self.repo()?;
        let refer = match param.branches {
            Some(ref branches) => {
                repo.find_branch(branches, git2::BranchType::Local)?.into_reference()
            }
            None => {
                repo.head()?
            }
        };
        let mut commit = match param.sha {
            Some(ref sha) => {
                repo.revparse_single(sha)?.peel_to_commit()?
            }
            None => {
                refer.peel_to_commit()?
            }
        };
        let mut diffs = Vec::new();
        let mut blobs = vec![];
        let tree = commit.tree()?;
        let mut path = param.path.clone();
        if path == "/" {
            path = "".to_string();
        }else if !path.ends_with("/") && !path.is_empty() {
            path += "/";
        }
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            if root != path {
                return TreeWalkResult::Ok;
            }
            match entry.kind() {
                Some(git2::ObjectType::Blob) => {
                    let path = format!("{}{}",root, entry.name().unwrap_or_default());
                    blobs.push((path,0)); return TreeWalkResult::Ok;
                }
                Some(git2::ObjectType::Tree) => {
                    let path = format!("{}{}",root, entry.name().unwrap_or_default());
                    blobs.push((path,1)); return TreeWalkResult::Ok;
                }
                _ => {
                    return TreeWalkResult::Skip;
                }
            }
        }).ok();
        loop {
            if diffs.len() == blobs.len() {
                break
            }
            let prev = match commit.parent(0) {
                Ok(commit) => {
                    commit
                }
                Err(_) => {
                    break;
                }
            };
            if prev.id() == commit.id() {
                break;
            }
            let df = repo.diff_tree_to_tree(
                Some(&prev.tree()?),
                Some(&commit.tree()?),
                None,
            )?;
            let commit_message = commit.message().unwrap_or_default();
            let author = commit.author().name().unwrap_or_default().to_string();
            let email = commit.author().email().unwrap_or_default().to_string();
            let date = commit.time().seconds();
            for delta in df.deltas() {
                let path = delta.new_file().path().unwrap().to_str().unwrap().to_string();
                let name = path.clone().split("/").last().unwrap_or_default().to_string();
                if blobs.iter().find(|&(p,i)| path.starts_with(p) && i == &1).is_some() {
                    // TODO 
                    // 
                    let mut path = path.split(&name).collect::<String>();
                    if path.ends_with("/") {
                        path = path.trim_end_matches("/").to_string();
                    }
                    if path.split("/").collect::<Vec<_>>().len() > 2 {
                        path = path.split("/").collect::<Vec<_>>()[0].to_string();
                    }else {
                        path = path.to_string();
                    }
                    let name = path.split("/").last().unwrap_or_default().to_string();
                    if diffs.iter().find(|x:&&GitMsgV2 |x.path == path).is_none() {
                        let id = delta.new_file().id();
                        let size = delta.new_file().size();
                        if diffs.len() > 20 {
                            break
                        }
                        diffs.push(GitMsgV2 {
                            sha1: id.to_string(),
                            name,
                            path,
                            size,
                            message: commit_message.to_string(),
                            cmt_id: commit.id().to_string(),
                            author: author.to_string(),
                            email: email.to_string(),
                            date,
                            r#type: "tree".to_string(),
                        }); 
                    }
                } else if blobs.contains(&(path.clone(),0)) {
                    if diffs.iter().find(|x:&&GitMsgV2 |x.path == path && x.name == name).is_none() {
                        let id = delta.new_file().id();
                        let size = delta.new_file().size();
                        diffs.push(GitMsgV2 {
                            sha1: id.to_string(),
                            name,
                            path,
                            size,
                            message: commit_message.to_string(),
                            cmt_id: commit.id().to_string(),
                            author: author.to_string(),
                            email: email.to_string(),
                            date,
                            r#type: "blob".to_string(),
                        });
                    }
                }
            }
            commit = prev;
        }
        Ok(diffs)
    }
}

#[test]
fn test_msg_v2() {
    use std::path::PathBuf;
    let mut git = GitParam::new(PathBuf::from("/home/zhenyi/文档"), "haproxy".to_string()).unwrap();
    let res = git.msg_v2(GitTreeParam {
        path: "".to_string(),
        branches: None,
        sha: None,
    });
    dbg!(res.unwrap());
}