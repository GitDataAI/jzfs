use sea_orm::*;
use crate::api::dto::check::CheckRepo;
use crate::api::service::check::CheckService;
use crate::metadata::model::groups::group;
use crate::metadata::model::users::users;

impl CheckService {
    pub async fn check_name(&self, name: String) -> anyhow::Result<CheckRepo>{
        let model = group::Entity::find()
            .filter(group::Column::Name.eq(name.clone()))
            .one(&self.db)
            .await?;
        if model.is_some(){
            return Ok(CheckRepo{
                exits: true,
                is_group: true
            })
        }
        let model = users::Entity::find()
            .filter(users::Column::Username.eq(name))
            .one(&self.db)
            .await?;
        if model.is_some(){
            return Ok(CheckRepo{
                exits: true,
                is_group: false
            })
        }
        Ok(CheckRepo{
            exits: false,
            is_group: false
        })
    }
}