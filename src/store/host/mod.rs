use crate::store::dto::{CommitDto, ObjectFile};
use git2::{Reference, Repository};
use time::OffsetDateTime;

pub struct GitLocal{
    pub uid: String,
    pub repo: Repository,
}


impl GitLocal {
    pub fn init(uid: String) -> Self{
        if std::fs::read_dir("./repos/").is_err(){
            std::fs::create_dir("./repos/").ok();
        }
        if Repository::open("./repos/".to_string() + &uid.to_string()).is_err(){
            Repository::init("./repos/".to_string() + &uid.to_string()).ok();
        }
        GitLocal{
            uid: uid.clone(),
            repo: Repository::open("./repos/".to_string() + &uid.to_string()).unwrap()
        }

    }
    pub fn head(&self) -> anyhow::Result<Reference> {
        let head = self.repo.head()?;
        Ok(head)
    }
    pub fn branchs(&self) -> anyhow::Result<Vec<String>> {
        let branch = self.repo.branches(None);
        if branch.is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let branch = branch?;
        let branchs = branch.flatten()
            .map(|x|x.0)
            .map(|x| x.into_reference())
            .filter(|x|x.is_branch())
            .map(|x| x.name().unwrap().to_string())
            .collect::<Vec<_>>();
        Ok(branchs)
    }
    pub fn commits_history(&self, branchs: String) -> anyhow::Result<Vec<CommitDto>> {
        let branch = self.repo.find_branch(branchs.as_str(), git2::BranchType::Local)?;
        let branch_commit = branch.get().peel_to_commit()?;
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(branch_commit.id())?;
        let mut cmxs = vec![];
        for id in revwalk {
            let commit = self.repo.find_commit(id?)?;
            cmxs.push(CommitDto{
                hash: commit.id().to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                date: OffsetDateTime::from_unix_timestamp(commit.time().seconds())?,
                branch: branchs.clone(),
            });
        }
        Ok(cmxs)
    }
    pub fn object_tree(&self, branch: String) -> anyhow::Result<Vec<ObjectFile>>{
        let head = self.repo.find_branch(branch.as_str(), git2::BranchType::Local);
        if head.is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let head = head?.into_reference();
        let head_commit = self.repo.find_commit(head.target().unwrap());
        let tree = head_commit?.tree()?;
        let mut clo = vec![];
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            clo.push(ObjectFile{
                root: root.to_string(),
                name: entry.name().unwrap_or("N/A").to_string(),
                hash: entry.id().to_string(),
            });
            0
        })?;
        Ok(clo)
    }
}


