use crate::endpoint::Endpoint;
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, Responder};
use cert::schema::{
    CertAuthLoginParam, CertEmailCaptchaParam, CertEmailCaptchaVerify, CertRegisterParam,
};
use web_session::builder::WebSession;

pub async fn users_login(
    param: Json<CertAuthLoginParam>,
    session: WebSession,
    endpoint: Data<Endpoint>,
    req: HttpRequest,
) -> impl Responder {
    endpoint.users_login(param.into_inner(), session, req).await
}

pub async fn users_register(
    param: Json<CertRegisterParam>,
    endpoint: Data<Endpoint>,
    session: WebSession,
    req: HttpRequest,
) -> impl Responder {
    endpoint
        .users_register(param.into_inner(), session, req)
        .await
}

pub async fn users_logout(
    endpoint: Data<Endpoint>,
    session: WebSession,
    req: HttpRequest,
) -> impl Responder {
    endpoint.users_logout(session, req).await
}

pub async fn email_captcha(
    endpoint: Data<Endpoint>,
    session: WebSession,
    param: Json<CertEmailCaptchaParam>,
) -> impl Responder {
    endpoint
        .email_captcha_send(param.into_inner(), session)
        .await
}
pub async fn email_verify(
    endpoint: Data<Endpoint>,
    session: WebSession,
    param: Json<CertEmailCaptchaVerify>,
) -> impl Responder {
    endpoint
        .email_captcha_verify(param.into_inner(), session)
        .await
}
