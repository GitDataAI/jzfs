use crate::api::app_write::AppWrite;
use crate::api::dto::user_dto::{UserKeyCreate, UserKeyList};
use crate::api::handler::check_session;
use crate::metadata::service::MetaService;
use actix_session::Session;
use actix_web::{web, Responder};
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/keys",
    responses(
        (status = 200, description = "Success", body = Vec<UserKeyList>),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_keys(
    session: Session,
    service: web::Data<MetaService>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<Vec<UserKeyList>>::unauthorized(model.err().unwrap().to_string())
    }
    match service.user_service().list_key(model.unwrap().uid).await{
        Ok(result)=>{
            AppWrite::ok(result)
        },
        Err(e)=>{
            AppWrite::fail(e.to_string())
        }
    }
}
#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/keys/{uid}",
    responses(
        (status = 200, description = "Success", body = UserKeyList),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_use_key_once(
    session: Session,
    service: web::Data<MetaService>,
    uid: web::Path<Uuid>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<UserKeyList>::unauthorized(model.err().unwrap().to_string());
    }
    match service.user_service().list_key(model.unwrap().uid).await{
        Ok(list)=>{
            for item in list.iter(){
                if item.uid == *uid{
                    return AppWrite::ok(item.clone())
                }
            }
            AppWrite::not_found("Not Found".to_string())
        },
        Err(e)=>{
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    post,
    tag = "user",
    path = "/api/v1/user/keys",
    request_body = UserKeyCreate,
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_key_create(
    session: Session,
    service: web::Data<MetaService>,
    dto: web::Json<UserKeyCreate>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<String>::unauthorized(model.err().unwrap().to_string());
    }
    match service.user_service().add_key(model.unwrap().uid, dto.name.clone(), dto.pubkey.clone()).await{
        Ok(_)=>{
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e)=>{
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    delete,
    tag = "user",
    path = "/api/v1/user/keys/{uid}",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_key_remove(
    session: Session,
    service: web::Data<MetaService>,
    uid: web::Path<Uuid>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<String>::unauthorized(model.err().unwrap().to_string());
    }
    match service.user_service().remove_key(model.unwrap().uid, uid.into_inner()).await{
        Ok(_)=>{
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e)=>{
            AppWrite::fail(e.to_string())
        }
    }
}

