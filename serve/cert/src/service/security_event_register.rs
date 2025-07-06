use authd::security;
use crate::schema::{AppResult, SecurityEventRegisterParam};
use crate::service::AppCertService;
use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

impl AppCertService {
    pub async fn service_security_event_register(
        &self,
        param: SecurityEventRegisterParam,
    ) -> AppResult<Uuid> {
        security::ActiveModel {
            uid: Set(Uuid::now_v7()),
            title: Set(param.title),
            description: Set(param.description),
            ip: Set(param.ip),
            user_agent: Set(param.user_agent),
            device: Set(param.device),
            location: Set(param.location),
            action: Set(param.action),
            actor: Set(param.actor),
            actor_uid: Set(param.actor_uid),
            user: Set(param.user),
            user_uid: Set(param.user_uid),
            timestamp: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map(|x| x.uid)
        .map(|x| AppResult {
            code: 200,
            data: Some(x),
            msg: None,
        })
        .unwrap_or(AppResult {
            code: 400,
            data: None,
            msg: None,
        })
    }
}
