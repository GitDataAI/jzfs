use crate::schema::AppResult;
use crate::service::AppCertService;
use authd::security;
use authd::security::Model;
use sea_orm::*;
use uuid::Uuid;

impl AppCertService {
    pub async fn service_security_event_list(&self, users_uid: Uuid) -> AppResult<Vec<Model>> {
        let result = security::Entity::find()
            .filter(authd::security::Column::Uid.eq(users_uid))
            .order_by_desc(authd::security::Column::Timestamp)
            .limit(1000)
            .all(&self.db)
            .await
            .ok();
        return AppResult {
            code: 200,
            data: result,
            msg: None,
        };
    }
}
