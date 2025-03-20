use crate::GitParam;
use git2::{Repository, Signature};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Deserialize, Serialize, Clone)]
pub struct GitCommit {
    pub id: String,
    pub msg: String,
    pub author: String,
    pub email: String,
    pub date: i64,
}

#[derive(Deserialize)]
pub struct CreateGitCommit {
    pub branches: String,
    pub user: String,
    pub email: String,
    pub msg: String,
    pub path: String,
    pub file: String,
    pub ops: i32, // 1 add 2 del todo: 3 modify
    pub context: Vec<u8>,
}

impl GitParam {
    pub fn list_commit(&mut self, branches: Option<String>) -> anyhow::Result<Vec<GitCommit>> {
        let repo = self.repo()?;
        let mut commits = vec![];
        let refs = match branches {
            Some(x) => match repo.find_branch(&*x, git2::BranchType::Local) {
                Ok(branches) => branches.into_reference(),
                Err(_) => return Err(anyhow::anyhow!("branch not found")),
            },
            None => repo.head()?,
        };

        let mut root = refs.peel_to_commit()?;
        commits.push(GitCommit {
            id: root.id().to_string(),
            msg: root.message().unwrap_or("N/A").to_string(),
            author: root.author().name().unwrap_or("N/A").to_string(),
            email: root.author().email().unwrap_or("N/A").to_string(),
            date: root.time().seconds(),
        });
        while let Ok(commit) = root.parent(0) {
            commits.push(GitCommit {
                id: commit.id().to_string(),
                msg: commit.message().unwrap_or("N/A").to_string(),
                author: commit.author().name().unwrap_or("N/A").to_string(),
                email: commit.author().email().unwrap_or("N/A").to_string(),
                date: commit.time().seconds(),
            });
            if root.parent(0).is_err() {
                break;
            }
            root = commit;
        }
        Ok(commits)
    }
    pub fn create_commit(&mut self, param: CreateGitCommit) -> anyhow::Result<()> {
        let branches = param.branches;
        let user = param.user;
        let email = param.email;
        let path = param.path;
        let file = param.file;
        let context = param.context;
        let msg = param.msg;
        let tempdir =
            tempdir::TempDir::new("jz-git").map_err(|_| anyhow::anyhow!("tempdir error"))?;
        let upstream = self.root.join(self.uid.clone());
        let repo = match Repository::clone(upstream.to_str().unwrap(), tempdir.path()) {
            Ok(repo) => repo,
            Err(_) => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("clone error"));
            }
        };
        match repo.set_head(&format!("refs/heads/{}", branches)) {
            Ok(_) => {}
            Err(_) => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("set_head error"));
            }
        }
        let parents = match repo.head() {
            Ok(head) => match head.peel_to_commit() {
                Ok(commit) => {
                    vec![commit]
                }
                Err(_) => {
                    vec![]
                }
            },
            Err(_) => {
                vec![]
            }
        };

        let time = chrono::Local::now().naive_local();
        let time = git2::Time::new(
            time.and_utc().timestamp(),
            time.and_utc().timestamp_subsec_nanos() as i32,
        );
        let sig = match Signature::new(&user, &email, &time) {
            Ok(sig) => sig,
            Err(_) => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("signature error"));
            }
        };
        let file_path = tempdir.path().join(path);
        if !file_path.exists() {
            match std::fs::create_dir_all(file_path.clone()) {
                Ok(_) => {}
                Err(_) => {
                    tempdir.close().ok();
                    return Err(anyhow::anyhow!("create_dir_all error"));
                }
            }
        }
        match param.ops {
            1 => {
                let mut file = match std::fs::File::create(file_path.join(file)) {
                    Ok(file) => file,
                    Err(_) => {
                        tempdir.close().ok();
                        return Err(anyhow::anyhow!("create_dir_all error"));
                    }
                };
                match file.write_all(&context) {
                    Ok(_) => {}
                    Err(_) => {
                        tempdir.close().ok();
                        return Err(anyhow::anyhow!("write_all error"));
                    }
                }
            }
            2 => match std::fs::remove_file(file_path.join(file)) {
                Ok(_) => {}
                Err(_) => {
                    tempdir.close().ok();
                    return Err(anyhow::anyhow!("remove_file error"));
                }
            },
            _ => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("ops error"));
            }
        }

        let tree_oid = match repo.index() {
            Ok(mut index) => {
                match index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None) {
                    Ok(_) => match index.write() {
                        Ok(_) => match index.write_tree() {
                            Ok(oid) => oid,
                            Err(_) => {
                                tempdir.close().ok();
                                return Err(anyhow::anyhow!("write_tree error"));
                            }
                        },
                        Err(_) => {
                            tempdir.close().ok();
                            return Err(anyhow::anyhow!("write error"));
                        }
                    },
                    Err(_) => {
                        tempdir.close().ok();
                        return Err(anyhow::anyhow!("add_all error"));
                    }
                }
            }
            Err(_) => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("index error"));
            }
        };
        let tree = match repo.find_tree(tree_oid) {
            Ok(tree) => tree,
            Err(_) => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("find_tree error"));
            }
        };
        let _commit_oid = match repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &tree,
            parents
                .iter()
                .map(|x| x)
                .collect::<Vec<&git2::Commit>>()
                .as_ref(),
        ) {
            Ok(oid) => oid,
            Err(_) => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("commit error"));
            }
        };
        let mut origin = match repo.find_remote("origin") {
            Ok(origin) => origin,
            Err(_) => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("find_remote error"));
            }
        };
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.push_update_reference(|_, status| {
            if status.is_some() {
                return Err(git2::Error::from_str("Failed to push"));
            }
            Ok(())
        });
        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(callbacks);
        match origin.push(
            &[format!("refs/heads/{}", branches)],
            Some(&mut push_options),
        ) {
            Ok(_) => {}
            Err(_) => {
                tempdir.close().ok();
                return Err(anyhow::anyhow!("push error"));
            }
        }
        tempdir.close().ok();
        Ok(())
    }
}
