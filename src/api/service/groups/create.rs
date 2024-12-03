use sea_orm::*;
use uuid::Uuid;
use crate::api::dto::group::GroupCreate;
use crate::api::service::groups::GroupService;
use crate::metadata::model::groups::group;

impl GroupService {
    pub async fn create(&self, dto: GroupCreate, owner: Uuid) -> anyhow::Result<bool>{
        let model = group::ActiveModel{
            uid: Set(Uuid::new_v4()),
            name: Set(dto.name),
            contact: Set(dto.contact),
            description: Set(dto.description),
            avatar: Default::default(),
            website: Default::default(),
            location: Default::default(),
            owner: Set(owner),
            created_at: Set(time::OffsetDateTime::now_utc()),
            updated_at: Set(time::OffsetDateTime::now_utc()),
            unit: Default::default(),
        };
        match model.insert(&self.db).await{
            Ok(_) => Ok(true),
            Err(e) => return Err(anyhow::anyhow!(e))
        }
    }
}