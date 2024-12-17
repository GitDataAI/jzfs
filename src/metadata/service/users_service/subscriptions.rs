use uuid::Uuid;
use crate::metadata::model::users::users_data;
use crate::metadata::service::users_service::UserService;
use sea_orm::*;
use crate::metadata::model::repo::repo;

impl UserService {
    pub async fn wacther(&self, uid: Uuid) -> anyhow::Result<Vec<Uuid>>{
        let model = users_data::Entity::find()
            .filter(users_data::Column::UserId.eq(uid))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Exist"))
        }
        let model = model.unwrap();
        Ok(model.watcher)
    }
    pub async fn wacthher_add(&self, uid: Uuid, target: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        let model = users_data::Entity::find()
            .filter(users_data::Column::UserId.eq(uid))
            .one(&txn)
            .await?;
        if model.is_none(){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("User Not Exist"))
        }
        let model = model.unwrap();
        if model.watcher.contains(&target){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("Already Exists"))
        }
        let mut watcher = model.watcher.clone();
        watcher.push(target);
        let mut arch = model.into_active_model();
        arch.watcher = Set(watcher);
        arch.update(&txn).await?;
        let target = repo::Entity::find()
            .filter(repo::Column::Uid.eq(target))
            .one(&txn)
            .await?;
        if target.is_none(){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("Target Not Exist"))
        }
        let target = target.unwrap();
        let mut watcher = target.watch.clone();
        let mut arch = target.into_active_model();
        watcher += 1;
        arch.watch = Set(watcher);
        arch.update(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
    pub async fn wacthher_remove(&self, uid: Uuid, target: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        let model = users_data::Entity::find()
            .filter(users_data::Column::UserId.eq(uid))
            .one(&txn)
            .await?;
        if model.is_none(){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("User Not Exist"))
        }
        let model = model.unwrap();
        if model.watcher.contains(&target){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("Already Exists"))
        }
        let mut watcher = model.watcher.clone();
        watcher.retain(|x| *x != target);
        let mut arch = model.into_active_model();
        arch.watcher = Set(watcher);
        arch.update(&txn).await?;
        let target = repo::Entity::find()
            .filter(repo::Column::Uid.eq(target))
            .one(&txn)
            .await?;
        if target.is_none(){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("Target Not Exist"))
        }
        let target = target.unwrap();
        let mut watcher = target.watch.clone();
        let mut arch = target.into_active_model();
        watcher -= 1;
        arch.watch = Set(watcher);
        arch.update(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
}