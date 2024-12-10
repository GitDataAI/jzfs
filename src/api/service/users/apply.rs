use crate::api::dto::users::UserApply;
use crate::api::service::users::UserService;
use uuid::Uuid;

impl UserService {
    pub async fn apply(&self, dto: UserApply) -> anyhow::Result<Uuid>{
        match self.txn.apply(dto).await{
            Ok(x) => {
                Ok(x)
            },
            Err(e) => Err(anyhow::anyhow!(e))
        }         
    }
}