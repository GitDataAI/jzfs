use crate::metadata::model::groups::groups;
use crate::metadata::service::groups_service::GroupService;
use sea_orm::*;
use uuid::Uuid;

impl GroupService {
    pub async fn check_name(&self, name: String) -> anyhow::Result<bool>{
        let model = groups::Entity::find()
            .filter(groups::Column::Name.eq(name.clone()))
            .filter(groups::Column::Username.eq(name))
            .one(&self.db)
            .await;
        match model {
            Ok(model) => {
                if model.is_none() {
                    return Ok(true);
                }
                Ok(false)
            },
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
    pub async fn name_to_uid(&self, name: String) -> anyhow::Result<Uuid>{
        let model = groups::Entity::find()
            .filter(groups::Column::Name.eq(name.clone()))
            .filter(groups::Column::Username.eq(name))
            .one(&self.db)
            .await;
        match model {
            Ok(model) => {
                match model {
                    Some(model) => {
                        Ok(model.uid)
                    },
                    None =>{
                        Err(anyhow::anyhow!("group not found"))
                    }
                }
            },
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
}