use sea_orm::{ActiveModelTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jz_model::{member, organization};
use crate::AppModule;

#[derive(Deserialize,Serialize)]
pub struct OrgCreate {
    pub name: String,
    pub description: Option<String>,
    pub owner_org: Option<String>,
}

impl AppModule {
    pub async fn create_org(&self, opsuid: Uuid, param: OrgCreate) -> anyhow::Result<()> {
        if param.name.is_empty() {
            return Err(anyhow::anyhow!("name is empty"));
        }
        if self.org_by_name(param.name.clone()).await.is_ok() {
            return Err(anyhow::anyhow!("name is exist"));
        }
        let ops = self.user_info_by_id(opsuid).await?;
        let org = organization::ActiveModel::new(
            param.name,
            ops.email,
            param.description,
            ops.uid,
            param.owner_org
        );
        let org_uid = org.clone().uid.unwrap();
        let member = member::ActiveModel::new(
            ops.uid,
            org_uid,
            99
        );
        let txn  = self.write.begin().await?;
        org.insert(&txn).await?;
        member.insert(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
}