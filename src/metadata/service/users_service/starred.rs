use sea_orm::{Set, TransactionTrait};
use uuid::Uuid;
use crate::metadata::model::repo::repo;
use crate::metadata::model::users::users_data;
use crate::metadata::service::users_service::UserService;
use sea_orm::*;

impl UserService {
    pub async fn star(&self, uid: Uuid) -> anyhow::Result<Vec<repo::Model>>{
        let model = users_data::Entity::find()
            .filter(users_data::Column::UserId.eq(uid))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("user not found"))
        }
        let star = model.unwrap().star;
        let models = repo::Entity::find()
            .filter(repo::Column::Uid.is_in(star))
            .all(&self.db)
            .await?;
        return Ok(models)
    }
    pub async fn instar(&self, uid: Uuid, repo_id: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        {
            let model = users_data::Entity::find()
                .filter(users_data::Column::UserId.eq(uid))
                .one(&txn)
                .await?;
            if model.is_none(){
                txn.rollback().await?;
                return Err(anyhow::anyhow!("user not found"))
            }
            let model = model.unwrap();
            let mut star = model.star.clone();
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
            let model = users_data::Entity::find()
                .filter(users_data::Column::UserId.eq(uid))
                .one(&txn)
                .await?;
            if model.is_none(){
                txn.rollback().await?;
                return Err(anyhow::anyhow!("user not found"))
            }
            let model = model.unwrap();
            let mut star = model.star.clone();
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