use uuid::Uuid;
use crate::api::service::repos::RepoService;
use crate::store::host::GitLocal;

impl RepoService {
    pub async fn once_files(&self, repo_id: Uuid, branch: String, hash: String ,path: String) -> anyhow::Result<Vec<u8>>{
        let store = GitLocal::init(repo_id.to_string());
        match store.object_file(branch, hash, path){
            Ok(content)=>{
                Ok(content)
            }
            Err(e)=>{
                Err(e)
            }
        }
    }
}