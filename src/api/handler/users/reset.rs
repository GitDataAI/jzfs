use actix_session::Session;
use actix_web::{web, Responder};
use base64::Engine;
use crate::api::dto::users::{UserResetPasswd, UserResetPassword, UsersInner};
use crate::api::middleware::session::{SessionModel, SESSION_USER_KEY};
use crate::api::service::Service;
use crate::utils::r::R;

pub async fn api_user_reset_passwd_profile(
    session: Session,
    dto: web::Json<UsersInner>,
    service: web::Data<Service>
) -> impl Responder
{
    let model = session.get::<SessionModel>(SESSION_USER_KEY).unwrap();
    if model.is_none(){
        return R::<String>{
            code: 400,
            msg: Option::from(
                "[Error] User Not Login".to_string()
            ),
            data: None,
        }
    }
    let model = model.unwrap();
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
    let dto: UserResetPasswd = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    match service.users.reset(dto,model.uid).await{
        Ok(_) => {
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

pub async fn api_user_reset_passwd_forget(
    dto: web::Json<UsersInner>,
    service: web::Data<Service>,
)
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
    let dto: UserResetPassword = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    match service.users.reset_by_token(dto).await{
        Ok(_) => {
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
