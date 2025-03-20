use serde_json::{json, Value};
use uuid::Uuid;
use crate::AppModule;

pub struct RepoAccessItem {
    pub owner: bool,
    pub owner_uid: Uuid,
    pub group: bool,
    pub repo_name: String,
    pub repo_uid: Uuid,
    pub access: i32,
}


impl AppModule {
    pub async fn repo_owners(&self, users_uid: Uuid) -> anyhow::Result<Vec<Value>> {
        let owner = self.user_info_by_id(users_uid).await?;
        let mut result = vec![];
        let repository = self.repo_info_by_owner_uid(owner.uid).await?;
        let repos = repository
            .iter().map(|x|{
            json!({
                "id": x.uid,
                "name": x.name,
                "description": x.description,
            })
        })
        .collect::<Vec<Value>>();
        result.push(json!({
            "uid": owner.uid,
            "username": owner.username,
            "avatar": owner.avatar,
            "list": repos,
            "owner": true,
            "group": false,
        }));
        let member = self.member_orgs(users_uid).await?;
        for org in member {
            let repo = self.repo_info_by_owner_uid(org.uid).await.unwrap_or_else(|_| vec![]);
            let repos = repo
                .iter().map(|x|{
                json!({
                    "id": x.uid,
                    "name": x.name,
                    "description": x.description,
                })
            })
            .collect::<Vec<Value>>();
            result.push(json!({
                "uid": org.uid,
                "username": org.name,
                "avatar": org.avatar,
                "list": repos,
                "owner": false,
                "group": true,
            }));
        }
        Ok(result)
    }
    
    pub async fn repo_access(&self, users_uid: Uuid) -> anyhow::Result<Vec<RepoAccessItem>> {
        let owner = self.user_info_by_id(users_uid).await?;
        let mut result = vec![];
        let repository = self.repo_info_by_owner_uid(owner.uid).await?;
        for x in repository {
            result.push(RepoAccessItem {
                owner: true,
                owner_uid: owner.uid,
                group: false,
                repo_name: x.name.clone(),
                repo_uid: x.uid,
                access: 0,
            });
        }
        for org in self.member_orgs(users_uid).await? {
            let repo = self.repo_info_by_owner_uid(org.uid).await?;
            for x in repo {
                result.push(RepoAccessItem {
                    owner: false,
                    owner_uid: org.uid,
                    group: true,
                    repo_name: x.name.clone(),
                    repo_uid: x.uid,
                    access: 0,
                });
            }
        }
        Ok(result)
    }
    
}