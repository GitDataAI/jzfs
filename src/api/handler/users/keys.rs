use crate::api::dto::users::UserKeyCreate;
use crate::api::service::Service;
use crate::utils::r::R;
use actix_session::Session;
use actix_web::{web, Responder};
use uuid::Uuid;

#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/key",
    request_body = UserKeyCreate,
    responses(
            (status = 200, description = "OK"),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 403, description = "Add Key Failed"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_users_key_create(
    session: Session,
    service: web::Data<Service>,
    dto: web::Json<UserKeyCreate>
)
-> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<String>{
            code: 402,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    match service.users.add_key(model.unwrap().uid, dto.name.clone(), dto.pubkey.clone()).await{
        Ok(_)=>{
            R{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e)=>{
            R{
                code: 405,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}

#[utoipa::path(
    delete,
    tag = "users",
    path = "/api/v1/users/key/{uid}",
    params(
        ("uid" = Uuid, description = "key Uid"),
    ),
    responses(
            (status = 200, description = "OK"),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 403, description = "Remove Key Failed"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_users_key_remove(
    session: Session,
    service: web::Data<Service>,
    uid: web::Path<Uuid>
)
-> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<String>{
            code: 402,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    match service.users.remove_key(model.unwrap().uid, uid.into_inner()).await{
        Ok(_)=>{
            R{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e)=>{
            R{
                code: 405,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}