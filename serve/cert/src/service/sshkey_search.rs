use crate::models::{sshkey, users};
use crate::schema::{AppResult, SshKeySearch};
use crate::service::AppCertService;
use sea_orm::*;

impl AppCertService {
    pub async fn service_sshkey_search(&self, param: SshKeySearch) -> AppResult<users::Model> {
        let model = sshkey::Entity::find()
            .filter(sshkey::Column::Content.eq(param.public_key))
            .one(&self.db)
            .await;
        match model {
            Ok(Some(model)) => {
                let user = users::Entity::find()
                    .filter(users::Column::Uid.eq(model.user_id))
                    .one(&self.db)
                    .await;
                if let Ok(u) = user {
                    return AppResult {
                        code: 200,
                        data: u,
                        msg: None,
                    };
                }
                AppResult {
                    code: 400,
                    data: None,
                    msg: None,
                }
            }
            Ok(None) => AppResult {
                code: 401,
                data: None,
                msg: None,
            },
            Err(err) => AppResult {
                code: 500,
                data: None,
                msg: Some(err.to_string()),
            },
        }
    }
}
