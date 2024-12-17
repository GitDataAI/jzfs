use uuid::Uuid;
use sea_orm::*;
use time::OffsetDateTime;
use crate::api::dto::user_dto::UserUpdate;
use crate::metadata::model::users::users;
use crate::metadata::service::users_service::UserService;

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
            model.description = Set(Some(description));
        }
        if let Some(company) = dto.company{
            model.company = Set(company);
        }
        if let Some(website) = dto.website{
            model.website = Set(website);
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