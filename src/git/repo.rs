use std::path::PathBuf;
use std::process::Command;
use git2::Repository;
use uuid::Uuid;
use crate::metadata::model::repo::repo;
use crate::ROOT_PATH;


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
    pub fn new(path: PathBuf, uid: Uuid, name: String) -> Self{
        let repo = Repository::open(path.clone()).unwrap();
        Self{
            path,
            uid,
            name,
            repo
        }
    }
    pub fn repo(&self) -> Repository{
        Repository::open(self.path.clone()).unwrap()
    }
    pub fn create(uid: Uuid) -> anyhow::Result<()>{
        let path = PathBuf::from(ROOT_PATH).join(uid.to_string());
        if path.exists(){
            std::fs::remove_dir(path.clone())?;
        }
        std::fs::create_dir_all(path.clone())?;
        Command::new("git")
            .arg("init")
            .arg("--bare")
            .current_dir(path.clone())
            .spawn()?;
        Ok(())
    }
}