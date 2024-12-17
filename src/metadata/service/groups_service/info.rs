use crate::api::dto::ListOption;
use crate::metadata::model::groups::groups;
use crate::metadata::service::groups_service::GroupService;
use sea_orm::*;
impl GroupService {
    pub async fn query(&self, key: String, list: ListOption) -> anyhow::Result<Vec<groups::Model>>{
        let models = groups::Entity::find()
            .filter(groups::Column::Name.contains(key.clone()))
            .filter(groups::Column::Description.contains(key.clone()))
            .filter(groups::Column::Username.contains(key))
            .filter(groups::Column::IsGroups.eq(true))
            .offset(list.page)
            .limit(list.size)
            .all(&self.db)
            .await;
        match models{
            Ok(models) => {
                Ok(models)
            },
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
    pub async fn info(&self, name: String) -> anyhow::Result<groups::Model>{
        let model = groups::Entity::find()
            .filter(groups::Column::Username.eq(name))
            .one(&self.db)
            .await;
        match model{
            Ok(model) => {
                match model{
                    Some(model) => {
                        Ok(model)
                    },
                    None => {
                        Err(anyhow::anyhow!("Not Found"))
                    }
                }
            },
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
}