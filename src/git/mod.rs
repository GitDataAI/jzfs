use crate::git::git::GitLocal;
use std::path::PathBuf;

pub mod ftp;
pub mod git;
pub mod http;
pub mod s3;
pub mod ssh;

#[derive(Clone)]
pub struct GitServer {
    pub root: PathBuf,
}

impl GitServer {
    pub fn new(root: PathBuf) -> Self {
        if !root.exists() {
            match std::fs::create_dir_all(&root) {
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            }
        }
        GitServer { root }
    }
    pub fn create_repo(&self, path: String) -> Result<GitLocal, Box<dyn std::error::Error>> {
        let repo = git2::Repository::init_bare(self.root.join(path))?;
        Ok(GitLocal { repository: repo })
    }
    pub fn open_repo(&self, path: String) -> Result<GitLocal, Box<dyn std::error::Error>> {
        let repo = git2::Repository::open_bare(self.root.join(path))?;
        Ok(GitLocal { repository: repo })
    }
    pub fn list_repo(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut result = vec![];
        for entry in std::fs::read_dir(self.root.clone())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                result.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
        Ok(result)
    }
    pub fn remove_repo(&self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::remove_dir_all(self.root.join(path))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    #[test]
    fn new_root_exists_should_not_create_directory() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        assert!(path.exists());
        let git_server = GitServer::new(path);
        assert!(git_server.root.exists());
    }

    #[test]
    fn new_root_does_not_exist_should_create_directory() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("nonexistent");
        assert!(!path.exists());
        let git_server = GitServer::new(path);
        assert!(git_server.root.exists());
    }

    #[test]
    fn create_repo_successful_creation_should_create_repository() {
        let temp_dir = TempDir::new().unwrap();
        let git_server = GitServer::new(temp_dir.path().to_path_buf());
        let repo_path = "test_repo";
        assert!(!git_server.root.join(repo_path).exists());
        git_server.create_repo(repo_path.to_string()).unwrap();
        assert!(git_server.root.join(repo_path).exists());
    }

    #[test]
    fn open_repo_repository_exists_should_open_repository() {
        let temp_dir = TempDir::new().unwrap();
        let git_server = GitServer::new(temp_dir.path().to_path_buf());
        let repo_path = "test_repo";
        git_server.create_repo(repo_path.to_string()).unwrap();
        let repo = git_server.open_repo(repo_path.to_string()).unwrap();
        assert!(repo.repository.path().exists());
    }

    #[test]
    fn list_repo_repositories_exist_should_list_repositories() {
        let temp_dir = TempDir::new().unwrap();
        let git_server = GitServer::new(temp_dir.path().to_path_buf());
        let repo_path1 = "test_repo1";
        let repo_path2 = "test_repo2";
        git_server.create_repo(repo_path1.to_string()).unwrap();
        git_server.create_repo(repo_path2.to_string()).unwrap();
        let repos = git_server.list_repo().unwrap();
        assert!(repos.contains(&repo_path1.to_string()));
        assert!(repos.contains(&repo_path2.to_string()));
    }

    #[test]
    fn remove_repo_repository_exists_should_remove_repository() {
        let temp_dir = TempDir::new().unwrap();
        let git_server = GitServer::new(temp_dir.path().to_path_buf());
        let repo_path = "test_repo";
        git_server.create_repo(repo_path.to_string()).unwrap();
        assert!(git_server.root.join(repo_path).exists());
        git_server.remove_repo(repo_path.to_string()).unwrap();
        assert!(!git_server.root.join(repo_path).exists());
    }
}
