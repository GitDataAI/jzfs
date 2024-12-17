use crate::metadata::model::groups::groups_labels;
use crate::metadata::service::groups_service::GroupService;
use uuid::Uuid;
use sea_orm::*;
impl GroupService {
    pub async fn labels(&self, group_id: Uuid) -> anyhow::Result<Vec<groups_labels::Model>>{
        let result = groups_labels::Entity::find()
            .filter(groups_labels::Column::GroupId.eq(group_id))
            .all(&self.db)
            .await;
        match result{
            Ok(models) => Ok(models),
            Err(e) => Err(anyhow::anyhow!("get group labels error:{}",e))
        }
    }
    pub async fn label_create(&self, group_id: Uuid, label: String, color: String) -> anyhow::Result<groups_labels::Model>{
        let result = groups_labels::ActiveModel{
            uid: Set(Uuid::new_v4()),
            label: Set(label),
            color: Set(color),
            group_id: Set(group_id),
            ..Default::default()
        }.insert(&self.db).await;
        match result{
            Ok(model) => Ok(model),
            Err(e) => Err(anyhow::anyhow!("create group label error:{}",e))
        }
    }
    pub async fn label_delete(&self, uid: Uuid) -> anyhow::Result<DeleteResult>{
        let result = groups_labels::Entity::delete_by_id(uid).exec(&self.db).await;
        match result{
            Ok(result) => Ok(result),
            Err(e) => Err(anyhow::anyhow!("delete group label error:{}",e))
        }
    }
    pub async fn label_update(&self, uid: Uuid, label: String, color: String) -> anyhow::Result<groups_labels::Model>{
        let result = groups_labels::Entity::find_by_id(uid).one(&self.db).await;
        match result {
            Ok(model) => {
                if model.is_none() {
                    return Err(anyhow::anyhow!("group label not found"));
                }
                let model = model.unwrap();
                let result = groups_labels::ActiveModel {
                    uid: Set(model.uid),
                    label: Set(label),
                    color: Set(color),
                    group_id: Set(model.group_id),
                    ..Default::default()
                }.update(&self.db).await;
                match result {
                    Ok(model) => Ok(model),
                    Err(e) => Err(anyhow::anyhow!("update group label error:{}",e))
                }
            },
            Err(e) => {
                Err(anyhow::anyhow!("get group label error:{}",e))
            }
        }
    }
    pub async fn label_check(&self, group_id: Uuid, label: String) -> anyhow::Result<bool>{
        let result = groups_labels::Entity::find()
            .filter(groups_labels::Column::GroupId.eq(group_id))
            .filter(groups_labels::Column::Label.eq(label))
            .one(&self.db)
            .await;
        match result{
            Ok(model) => Ok(model.is_some()),
            Err(e) => Err(anyhow::anyhow!("check group label error:{}",e))
        }
    }
}