use crate::api::dto::user_dto::UserFollowerOv;
use crate::metadata::model::users::{users, users_data};
use crate::metadata::service::users_service::UserService;
use sea_orm::*;

impl UserService {
    pub async fn followers(&self, uid: uuid::Uuid) -> anyhow::Result<Vec<UserFollowerOv>> {
        let model = users_data::Entity::find()
            .filter(
                users_data::Column::UserId.eq(uid)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        let models = users::Entity::find()
            .filter(
                users::Column::Uid.is_in(model.follow)
            )
            .all(&self.db)
            .await?
            .iter()
            .map(|x|{
                UserFollowerOv{
                    uid: x.uid.clone(),
                    name: x.name.clone(),
                    username: x.username.clone(),
                    avatar: None,
                    description: x.description.clone(),
                }
            })
            .collect::<Vec<_>>();
        Ok(models)
    }
    pub async fn followed(&self, uid: uuid::Uuid) -> anyhow::Result<Vec<UserFollowerOv>> {
        let model = users_data::Entity::find()
            .filter(
                users_data::Column::UserId.eq(uid)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        let models = users::Entity::find()
            .filter(
                users::Column::Uid.is_in(model.following)
            )
            .all(&self.db)
            .await? 
            .iter()
            .map(|x|{
                UserFollowerOv{
                    uid: x.uid.clone(),
                    name: x.name.clone(),
                    username: x.username.clone(),
                    avatar: x.avatar.clone(),
                    description: x.description.clone(),
                }
            })
            .collect::<Vec<_>>();
        Ok(models)
    }
    pub async fn follow(&self, uid: uuid::Uuid, target: uuid::Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        let model = users_data::Entity::find()
            .filter(
                users_data::Column::UserId.eq(uid)
            )
            .one(&txn)
            .await?;
        if model.is_none(){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        let mut follow = model.clone().follow;
        follow.push(target);
        let mut arch = model.into_active_model();
        arch.follow = Set(follow);
        arch.update(&txn).await?;
        let target = users_data::Entity::find()
            .filter(
                users_data::Column::UserId.eq(target)
            )
            .one(&txn)
            .await?;
        if target.is_none(){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let mut following = target.clone().unwrap().following;
        following.push(uid);
        let mut arch = target.unwrap().into_active_model();
        arch.following = Set(following);
        arch.update(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
    pub async fn unfollow(&self, uid: uuid::Uuid, target: uuid::Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        let model = users_data::Entity::find()
            .filter(
                users_data::Column::UserId.eq(uid)
            )
            .one(&txn)
            .await?;
        if model.is_none(){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        let mut follow = model.clone().follow;
        follow.retain(|x|{
            *x != target
        });
        let mut arch = model.into_active_model();
        arch.follow = Set(follow);
        arch.update(&txn).await?;
        let target = users_data::Entity::find()
            .filter(
                users_data::Column::UserId.eq(target)
            )
            .one(&txn)
            .await?;
        if target.is_none(){
            txn.rollback().await?;
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let mut following = target.clone().unwrap().following;
        following.retain(|x|{
            *x != uid
        });
        let mut arch = target.unwrap().into_active_model();
        arch.following = Set(following);
        arch.update(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
}