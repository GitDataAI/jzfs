use lib_entity::ActiveModelTrait;
use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::QueryFilter;
use lib_entity::TransactionTrait;
use lib_entity::repos::repos;
use lib_entity::users::users;
use serde::Deserialize;
use uuid::Uuid;

use crate::GIT_ROOT;
use crate::service::AppFsState;

#[derive(Deserialize, Clone)]
pub struct CreateRepositoryParma {
    name : String,
    owner : Uuid,
    description : Option<String>,
    node : Uuid,
    readme : bool,
    idcrad : bool,
    private : bool,
    default_branch : Option<String>,
}
impl AppFsState {
    pub async fn create_repo(
        &self,
        users_uid : Uuid,
        parma : CreateRepositoryParma,
    ) -> anyhow::Result<()> {
        let txn = self.write.begin().await?;
        if users_uid != parma.owner {
            return Err(anyhow::anyhow!("permission denied"));
        }
        let (owner_uid, owner_name) = {
            let user = users::Entity::find()
                .filter(users::Column::Uid.eq(parma.owner))
                .one(&self.read)
                .await
                .map_err(|e| anyhow::anyhow!("{}", e))?
                .ok_or(anyhow::anyhow!("user not found"))?;
            (user.uid, user.name)
        };

        if owner_uid != parma.owner {
            return Err(anyhow::anyhow!("permission denied"));
        }

        let repo = repos::ActiveModel::new(
            owner_name,
            parma.owner,
            parma.private,
            parma.name.clone(),
            parma.description,
            parma.default_branch,
            parma.node,
        );
        match repo.clone().insert(&txn).await {
            Ok(_) => {}
            Err(e) => {
                txn.rollback().await?;
                return Err(anyhow::anyhow!("{}", e));
            }
        }
        let path = format!("{}/{}/{}", GIT_ROOT, owner_uid, repo.uid.unwrap());
        self.transport
            .server_create_repository(path.clone())
            .await?;
        if parma.readme {
            // TODO
        }
        if parma.idcrad {
            // TODO
        }
        Ok(())
    }
}
