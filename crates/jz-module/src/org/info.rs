use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use uuid::Uuid;
use jz_model::organization;
use crate::AppModule;

impl AppModule {
    pub async fn org_by_name(&self, name: String) -> anyhow::Result<organization::Model> {
        organization::Entity::find()
            .filter(organization::Column::Name.eq(name))
            .one(&self.read)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Org Not Found"))
    }
    pub async fn org_by_uid(&self, uid: Uuid) -> anyhow::Result<organization::Model> {
        organization::Entity::find_by_id(uid)
            .one(&self.read)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Org Not Found"))
    }
}