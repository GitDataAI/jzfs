use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;
use uuid::Uuid;
use jz_model_sqlx::{uuid_v4, uuid_v7};
use jz_model_sqlx::token::{TokenList, TokenModel, UsersTokenAccess};
use crate::app::AppService;

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersTokenCreate {
    name: String,
    description: Option<String>,
    expire: i64,
    access: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersTokenCreateResponse {
    uid: Uuid,
    token: String,
    expire: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersTokenDelete {
    uid: Uuid,
    name: String,
}


impl AppService {
    pub async fn users_token_create(
        &self,
        users_uid: Uuid,
        param: UsersTokenCreate,
    ) -> anyhow::Result<UsersTokenCreateResponse> {
        let token = format!("{}-{}-{}-{}", uuid_v4(), uuid_v7(), users_uid, Utc::now()).digest();
        let fingerprint = format!("{}-{}-{}-{}", uuid_v4(), uuid_v7(), users_uid, token);
        let now = Utc::now();
        let expire = if param.expire == 0 {
            now + chrono::Duration::days(365 * 10)
        } else {
            now + chrono::Duration::days(param.expire)
        };
        let active = UsersTokenCreateResponse {
            uid: Uuid::new_v4(),
            token: token.clone(),
            expire: expire.timestamp(),
        };
        let builder = TokenModel::builder()
            .description(param.description)
            .expires_at(expire)
            .fingerprint(fingerprint)
            .name(param.name)
            .token(token)
            .user_id(users_uid)
            .access(UsersTokenAccess::from_i64(param.access).to_string())
            .build()?;
        let mapper = self.token_mapper();
        if let Err(e) = mapper.insert(builder).await {
            return Err(anyhow::anyhow!("{}", e));
        }
        Ok(active)
    }
    pub async fn users_token_delete(
        &self,
        users_uid: Uuid,
        param: UsersTokenDelete,
    ) -> anyhow::Result<()> {
        let mapper = self.token_mapper();
        let model = mapper.query().query_by_uid(param.uid).await?;
        if model.user_id != users_uid {
            return Err(anyhow::anyhow!("token not found"));
        }
        if model.name != param.name {
            return Err(anyhow::anyhow!("token not found"));
        }
        if let Err(e) = mapper.relation(model).delete().await {
            return Err(anyhow::anyhow!("{}", e));
        }
        Ok(())
    }
    pub async fn users_token_list(&self, users_uid: Uuid) -> anyhow::Result<Vec<TokenList>> {
        let mapper = self.token_mapper();
        let models = mapper.query().query_by_user_id(users_uid).await?;
        Ok(models.into_iter().map(|m| TokenList::from(m)).collect())
    }
    pub async fn users_token_find(&self, token: String) -> anyhow::Result<TokenList> {
        let mapper = self.token_mapper();
        if let Ok(model) = mapper.query().query_by_token(token).await {
            return Ok(TokenList::from(model));
        }
        Err(anyhow::anyhow!("token not found"))
    }
}