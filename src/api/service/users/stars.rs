use sea_orm::*;
use uuid::Uuid;
use crate::api::service::users::UserService;
use crate::metadata::model::repos::repo;
use crate::metadata::model::users::users_other;

impl UserService {
    pub async fn star(&self, uid: Uuid) -> anyhow::Result<Vec<Uuid>>{
        let model = users_other::Entity::find_by_id(
            uid
        )
        .one(&self.db)
        .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("user not found"))
        }
        let star = model.unwrap().star;
        return Ok(star)
    } 
    pub async fn instar(&self, uid: Uuid, repo_id: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        {
            let mut star: Vec<Uuid> = Vec::new();
            let model = users_other::Entity::find_by_id(
                uid
            )
            .one(&txn)
            .await?;
            if model.is_none(){
                txn.rollback().await?;
                return Err(anyhow::anyhow!("user not found"))
            }
            let model = model.unwrap();
            star = model.star.clone();
            star.push(repo_id);
            let mut arch = model.into_active_model();
            arch.star = Set(star);
            match arch.update(&txn).await{
                Ok(_) => {},
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::Error::new(e))
                }
            }
            let repo_model = repo::Entity::find_by_id(repo_id)
                .one(&txn)
                .await?;
            if repo_model.is_none(){
                txn.rollback().await?;
                return Err(anyhow::anyhow!("repo not found"))
            }
            let mut repo_arch = repo_model.unwrap().into_active_model();
            let star = repo_arch.clone().star.unwrap() + 1;
            repo_arch.star = Set(star);
            match repo_arch.update(&txn).await{
                Ok(_) => {},
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::Error::new(e))
                }
            }
        }
        txn.commit().await?;
        Ok(())
    }
    pub async fn unstar(&self, uid: Uuid, repo_id: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        {
            let mut star: Vec<Uuid> = Vec::new();
            let model = users_other::Entity::find_by_id(
                uid
            )
                .one(&txn)
                .await?;
            if model.is_none(){
                txn.rollback().await?;
                return Err(anyhow::anyhow!("user not found"))
            }
            let model = model.unwrap();
            star = model.star.clone();
            star.retain(|x| *x != repo_id);
            let mut arch = model.into_active_model();
            arch.star = Set(star);
            match arch.update(&txn).await{
                Ok(_) => {},
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::Error::new(e))
                }
            }
            let repo_model = repo::Entity::find_by_id(repo_id)
                .one(&txn)
                .await?;
            if repo_model.is_none(){
                txn.rollback().await?;
                return Err(anyhow::anyhow!("repo not found"))
            }
            let mut repo_arch = repo_model.unwrap().into_active_model();
            let star = repo_arch.clone().star.unwrap() - 1;
            repo_arch.star = Set(star);
            match repo_arch.update(&txn).await{
                Ok(_) => {},
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::Error::new(e))
                }
            }
        }
        txn.commit().await?;
        Ok(())
    }
}