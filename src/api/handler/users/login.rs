use actix_session::Session;
use actix_web::{web, Responder};
use base64::Engine;
use crate::api::dto::users::{UsersInner, UsersLoginEmail, UsersLoginUsername};
use crate::api::middleware::session::SESSION_USER_KEY;
use crate::api::service::Service;
use crate::utils::r::R;

pub async fn api_users_login_name(
    session: Session,
    dto: web::Json<UsersInner>,
    service: web::Data<Service>)
    -> impl Responder
{
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    let dto: UsersLoginUsername = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    match service.users.login_by_username(dto).await{
        Ok(info) => {
            session.insert(SESSION_USER_KEY, info).unwrap();
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e) => {
            R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}
pub async fn api_users_login_email(
    session: Session,
    dto: web::Json<UsersInner>,
    service: web::Data<Service>)
    -> impl Responder
{
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    let dto: UsersLoginEmail = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    match service.users.login_by_email(dto).await{
        Ok(info) => {
            session.insert(SESSION_USER_KEY, info).unwrap();
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e) => {
            R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}