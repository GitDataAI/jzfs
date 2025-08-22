use anyhow::anyhow;
use config::git::AppGitConfig;
use database::entity::git_repo::Model;
use error::AppError;
use git2::Repository;
use std::path::PathBuf;

pub mod object;
pub mod service;
pub mod transport;

#[derive(Clone)]
pub struct GitContext {
    pub path_dir: PathBuf,
}

impl TryFrom<(Model, AppGitConfig)> for GitContext {
    type Error = AppError;
    fn try_from(value: (Model, AppGitConfig)) -> Result<Self, Self::Error> {
        let (model, config) = value;
        let repo_storage_name = model.storage.clone();
        if let Some(storage) = config.storage.iter().find(|x| x.name == repo_storage_name) {
            let path_dir = storage.path.join(model.uid.to_string());
            Ok(Self { path_dir })
        } else {
            Err(AppError::from(anyhow!("storage not found")))
        }
    }
}

impl GitContext {
    pub fn repo(&self) -> Result<Repository, AppError> {
        Repository::open_bare(self.path_dir.as_path()).map_err(|e| AppError::from(anyhow!(e)))
    }
    pub fn init(&self) -> Result<Repository, AppError> {
        Repository::init_bare(self.path_dir.as_path()).map_err(|e| AppError::from(anyhow!(e)))
    }
}
