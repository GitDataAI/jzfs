use git2::{Reference, Repository};

pub mod branch;
pub mod commit;
pub mod object;

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
  
}


