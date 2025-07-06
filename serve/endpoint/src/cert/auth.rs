use crate::endpoint::Endpoint;
use crate::utils::parse::user_agent_parse_os;
use actix_web::http::header::USER_AGENT;
use actix_web::{HttpRequest, HttpResponse};
use authd::security;
use cert::rpc::session::UsersSession;
use cert::schema::{CertAuthLoginParam, CertRegisterParam, SecurityEventRegisterParam};
use serde_json::json;
use web_session::builder::WebSession;

impl Endpoint {
    pub async fn users_login(
        &self,
        param: CertAuthLoginParam,
        web_session: WebSession,
        req: HttpRequest,
    ) -> HttpResponse {
        let context = self.new_context();
        let res = self.cert.user_auth_login(context, param).await;
        match res {
            Ok(res) => {
                if let Some(data) = res.data {
                    web_session.0.set(WebSession::USER_SESSION, data.clone());
                    let ip = req
                        .connection_info()
                        .realip_remote_addr()
                        .map(|x| x.to_string());
                    let user_agent = req
                        .headers()
                        .get(USER_AGENT)
                        .map(|x| x.to_str().unwrap_or("N/A").to_string());
                    let device = user_agent_parse_os(user_agent.clone());
                    let event = SecurityEventRegisterParam {
                        title: security::Model::USER_LOGIN.to_string(),
                        description: None,
                        ip,
                        user_agent,
                        device,
                        location: None,
                        action: "Cert Service".to_string(),
                        actor: data.username.clone(),
                        actor_uid: data.uid,
                        user: data.username,
                        user_uid: data.uid,
                    };
                    self.cert.security_event_register(context, event).await.ok();
                    HttpResponse::Ok().json(json!({ "code": res.code }))
                } else {
                    let msg = res.msg.unwrap_or("".to_string());
                    HttpResponse::Ok().json(json!({ "code": res.code, "msg": msg }))
                }
            }
            Err(err) => HttpResponse::Ok().json(json!({ "code": 501, "msg": err.to_string() })),
        }
    }
    pub async fn users_logout(&self, web_session: WebSession, req: HttpRequest) -> HttpResponse {
        let Ok(data) = web_session.0.get::<UsersSession>(WebSession::USER_SESSION) else {
            return HttpResponse::Ok().json(json!({ "code": 401, "msg": "unauthorized" }));
        };
        let context = self.new_context();
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .map(|x| x.to_string());
        let user_agent = req
            .headers()
            .get(USER_AGENT)
            .map(|x| x.to_str().unwrap_or("N/A").to_string());
        let device = user_agent_parse_os(user_agent.clone());
        let event = SecurityEventRegisterParam {
            title: security::Model::USER_LOGIN.to_string(),
            description: None,
            ip,
            user_agent,
            device,
            location: None,
            action: "Cert Service".to_string(),
            actor: data.username.clone(),
            actor_uid: data.uid,
            user: data.username,
            user_uid: data.uid,
        };
        web_session.0.remove(WebSession::USER_SESSION);
        self.cert.security_event_register(context, event).await.ok();
        HttpResponse::Ok().json(json!({ "code": 200 }))
    }
    pub async fn users_register(
        &self,
        param: CertRegisterParam,
        web_session: WebSession,
        req: HttpRequest,
    ) -> HttpResponse {
        let context = self.new_context();
        let res = self.cert.user_auth_register(context, param).await;
        match res {
            Ok(res) => {
                if let Some(data) = res.data {
                    web_session.0.set(WebSession::USER_SESSION, data.clone());
                    let ip = req
                        .connection_info()
                        .realip_remote_addr()
                        .map(|x| x.to_string());
                    let user_agent = req
                        .headers()
                        .get(USER_AGENT)
                        .map(|x| x.to_str().unwrap_or("N/A").to_string());
                    let device = user_agent_parse_os(user_agent.clone());
                    let event = SecurityEventRegisterParam {
                        title: security::Model::USER_LOGIN.to_string(),
                        description: None,
                        ip,
                        user_agent,
                        device,
                        location: None,
                        action: "Cert Service".to_string(),
                        actor: data.username.clone(),
                        actor_uid: data.uid,
                        user: data.username,
                        user_uid: data.uid,
                    };
                    self.cert.security_event_register(context, event).await.ok();
                    HttpResponse::Ok().json(json!({ "code": res.code }))
                } else {
                    let msg = res.msg.unwrap_or("".to_string());
                    HttpResponse::Ok().json(json!({ "code": res.code, "msg": msg }))
                }
            }
            Err(err) => HttpResponse::Ok().json(json!({ "code": 501, "msg": err.to_string() })),
        }
    }
}
