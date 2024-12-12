use uuid::Uuid;
use crate::api::service::Service;

pub async fn check_repo_owner(service: &Service, uid: Uuid, repo_id: Uuid) -> anyhow::Result<bool>{
    let repo = service.repo.owner(uid).await?;
    if repo.iter().any(|x| x.uid == repo_id){
        Ok(true)
    }else {
        let groups = service.group.check_member(repo_id,2).await?;
        let group_ids = groups
            .iter()
            .map(|x| x.uid)
            .collect::<Vec<_>>();
        let mut repos = vec![];
        for group_id in group_ids {
            let repo = service.repo.repo_by_group(group_id).await?;
            repos.extend(repo);
        }
        if repos.iter().any(|x| x.uid == repo_id){
            Ok(true)
        }else{
            Ok(false)
        }
    }
}