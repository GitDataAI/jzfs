use uuid::Uuid;
use crate::api::dto::users::UserUpdate;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users;
use sea_orm::*;
use time::OffsetDateTime;

impl UserService {
    pub async fn update_by_uid(&self, uid: Uuid, dto: UserUpdate) -> anyhow::Result<()>{
        let model = users::Entity::find()
            .filter(
                users::Column::Uid.eq(uid)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let mut model = model.unwrap().into_active_model();
        if let Some(name) = dto.name{
            model.name = Set(name);
        }
        if let Some(username) = dto.username{
            model.username = Set(username);
        }
        if let Some(phone) = dto.phone{
            model.phone = Set(Option::from(phone));
        }
        if let Some(description) = dto.description{
            model.description = Set(description);
        }
        if let Some(company) = dto.company{
            model.company = Set(company);
        }
        if let Some(website) = dto.website{
            model.website = Set(website);
        }
        if let Some(sex) = dto.sex{
            model.sex = Set(Some(sex));
        }
        if let Some(localtime) = dto.localtime{
            model.localtime = Set(localtime);
        }
        if let Some(timezone) = dto.timezone{
            model.timezone = Set(timezone);
        }
        if let Some(theme) = dto.theme{
            model.theme = Set(theme);
        }
        
        model.updated_at = Set(OffsetDateTime::now_utc());
        model.update(&self.db).await?;
        Ok(())
    }
}