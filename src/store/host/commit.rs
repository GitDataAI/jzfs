use git2::DiffOptions;
use time::{format_description, OffsetDateTime};
use crate::store::dto::{CommitDto, ObjectFile};
use crate::store::host::GitLocal;

impl GitLocal {
    pub fn commits_history(&self, branchs: String) -> anyhow::Result<Vec<CommitDto>> {
        let branch = self.repo.find_branch(branchs.as_str(), git2::BranchType::Local)?;
        let branch_commit = branch.get().peel_to_commit()?;
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(branch_commit.id())?;
        let mut cmxs = vec![];
        for id in revwalk {
            let commit = self.repo.find_commit(id?)?;
            let parent = commit.parent(0);
            if parent.is_err(){
                continue;
            }
            let mut opts = DiffOptions::new();
            opts.include_untracked(false);
            let parent_commit = self.repo.diff_tree_to_tree(
                Some(&parent?.tree()?),
                Some(&commit.tree()?),
                Some(&mut opts)
            );
            if parent_commit.is_err(){
                continue;
            }
            let mut files = vec![];
            for delta in parent_commit?.deltas() {
                let status = delta.status();
                let path = delta.new_file().path().unwrap().to_str().unwrap().to_string();
                if path.contains(".gitignore"){
                    continue;
                }
                let name = path.split("/").collect::<Vec<&str>>().last().map(|x| x.to_string());
                if name.is_none(){
                    continue;
                }
                let root = path.split("/").collect::<Vec<&str>>()[0..path.split("/").collect::<Vec<&str>>().len()-1].join("/");
                let hash = delta.new_file().id().to_string();
                match status {
                    git2::Delta::Added => {
                        files.push(ObjectFile{
                            root,
                            name: name.unwrap(),
                            hash,
                        })
                    },
                    git2::Delta::Deleted => {
                        files.push(ObjectFile{
                            root,
                            name: name.unwrap(),
                            hash,
                        })
                    },
                    git2::Delta::Modified => {
                        files.push(ObjectFile{
                            root,
                            name: name.unwrap(),
                            hash,
                        })
                    },
                    _=> {
                        
                    },
                }
            }
            cmxs.push(CommitDto{
                hash: commit.id().to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                date: OffsetDateTime::from_unix_timestamp(commit.time().seconds())?.format(
                    &format_description::parse(
                        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
                    )?
                )?,
                branch: branchs.clone(),
                unix: commit.time().seconds(),
                files,
            });
        }
        Ok(cmxs)
    }
}