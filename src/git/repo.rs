use crate::metadata::model::repo::repo;
use crate::ROOT_PATH;
use git2::{BranchType, IndexEntry, IndexTime, Repository};
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;


pub struct GitRepo{
    pub path: PathBuf,
    pub uid: Uuid,
    pub name: String,
    pub repo: Repository
}


impl From<repo::Model> for GitRepo {
    fn from(value: repo::Model) -> Self {
        let current = PathBuf::from(ROOT_PATH);
        let path = current.join(value.uid.to_string());
        let repo = Repository::open(path.clone()).unwrap();
        Self{
            path,
            uid: value.uid,
            name: value.name,
            repo
        }
    }
}

impl GitRepo{
    pub fn new(path: PathBuf, uid: Uuid, name: String) -> anyhow::Result<Self>{
        let repo = Repository::open(path.clone())?;
        Ok(Self{
            path,
            uid,
            name,
            repo
        })
    }
    pub fn repo(&self) -> Repository{
        Repository::open(self.path.clone()).unwrap()
    }
    pub fn create(uid: Uuid) -> anyhow::Result<()> {
        let path = PathBuf::from(ROOT_PATH).join(uid.to_string());
        if path.exists() {
            std::fs::remove_dir_all(path.clone())?;
        }
        std::fs::create_dir_all(path.clone())?;

        let mut child = Command::new("git")
            .arg("init")
            .arg("--bare")
            .current_dir(path.clone())
            .spawn()?;

        let status = child.wait()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to initialize git repository"));
        }

        Ok(())
    }

    pub fn files(&self, branchs: String, commit_id: Option<String>, file_path: String) -> anyhow::Result<Vec<u8>>{
        let repo = self.repo();
        let branch = repo.find_branch(branchs.as_str(), git2::BranchType::Local)?;
        let commit = match commit_id{
            Some(id)=> repo.find_commit(id.parse::<git2::Oid>()?)?,
            None=> branch.get().peel_to_commit()?
        };
        let tree = commit.tree()?;
        let entry = tree.get_path(&*PathBuf::from(file_path));
        if entry.is_err(){
            return Err(anyhow::anyhow!("file not found"))
        }
        let entry = entry?;
        let blob = entry.to_object(&self.repo)?.as_blob().map(|x|x.clone()).ok_or_else(||{
            anyhow::anyhow!("file not found")
        });
        Ok(blob?.content().to_vec())
    }
    pub fn readme(&self, branch: String) -> anyhow::Result<Vec<u8>>{
        let readme = self.files(branch, None, "README.md".to_string());
        if readme.is_err(){
            return Err(anyhow::anyhow!("readme not found"))
        }
        Ok(readme?)
    }
    pub fn create_branch(&self, branch_name: &str, start_point: Option<&str>) -> anyhow::Result<()> {
        let repo = &self.repo;

        // 检查分支是否已存在
        if repo.find_branch(branch_name, BranchType::Local).is_ok() {
            return Err(anyhow::anyhow!("Branch '{}' already exists", branch_name));
        }

        let start_point_oid = match start_point {
            Some(point) => repo.revparse_single(point)?.id(),
            None => {
                let head = repo.head()?;
                head.peel_to_commit()?.id()
            }
        };
        repo.branch(
            branch_name,
            &repo.find_object(start_point_oid, None).unwrap().peel_to_commit()?, false
        )?;
        Ok(())
    }
    pub fn add_file(
        &self,
        branch: String,
        path: String,
        file_name: String,
        content: Vec<u8>,
        msg: String,
        username: String,
        email: String,
    ) -> anyhow::Result<()> {
        let repo = self.repo();

        let branch = repo.find_branch(branch.as_str(), BranchType::Local)?;
        let commit = branch.get().peel_to_commit()?;
        let tree = commit.tree()?;

        let path = PathBuf::from(format!("{}/{}", path, file_name));

        let mut index = repo.index()?;
        index.read_tree(&tree)?;

        let times = time::OffsetDateTime::now_utc();
        let time = IndexTime::new(times.unix_timestamp() as i32, times.nanosecond());

        let blob_oid = repo.blob(content.as_slice())?;

        let mut entry = IndexEntry {
            ctime: time,
            mtime: time,
            dev: 0,
            ino: 0,
            mode: 0o100644,
            uid: 0,
            gid: 0,
            file_size: content.len() as u32,
            id: blob_oid,
            flags: (path.to_string_lossy().len() as u16) & 0xFFF,
            flags_extended: 0,
            path: path.to_string_lossy().as_bytes().to_vec(),
        };

        index.add_frombuffer(&mut entry, content.as_slice())?;
        index.write()?;

        let tree_oid = index.write_tree()?;
        let new_tree = repo.find_tree(tree_oid)?;

        let time: i64 = times.unix_timestamp();
        let offset_seconds = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC).whole_seconds();
        let signature = git2::Signature::new(
            username.as_str(),
            email.as_str(),
            &git2::Time::new(time, offset_seconds),
        )?;

        let branch_name = branch.name()?.ok_or_else(|| anyhow::anyhow!("Branch name not found"))?;
        repo.commit(
            Option::from(branch_name),
            &signature,
            &signature,
            msg.as_str(),
            &new_tree,
            &[&commit],
        )?;

        Ok(())
    }
}
