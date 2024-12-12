use uuid::Uuid;
use crate::api::service::repos::RepoService;
use crate::store::dto::CommitDto;
use crate::store::host::GitLocal;

impl RepoService {
    pub fn commit_history(&self, repo_id: Uuid, branch: String) -> anyhow::Result<Vec<CommitDto>>{
        let store = GitLocal::init(repo_id.to_string());
        match store.commits_history(branch){
            Ok(cmxs) => Ok(cmxs),
            Err(e) => Err(e)
        }
    }
}