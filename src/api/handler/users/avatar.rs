use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::dto::users::UserAvatar;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/avatar",
    request_body = UserAvatar,
    responses(
            (status = 200, description = "OK"),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 403, description = "Upload Avatar Failed"),
            (status = 405, description ="Other Error"),
    )
)]
pub async fn api_user_avatar_upload(
    session: Session,
    service: web::Data<Service>,
    dto: web::Json<UserAvatar>
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
    match service.users.upload_avatar(model.unwrap().uid,dto.byte.clone()).await{
        Ok(result)=>{
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
    path = "/api/v1/users/avatar",
    responses(
            (status = 200, description = "OK"),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 403, description = "Delete Avatar Failed"),
            (status = 405, description ="Other Error"),
    )
)]
pub async fn api_user_avatar_delete(
    session: Session,
    service: web::Data<Service>,
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
    match service.users.delete_avatar(model.unwrap().uid).await{
        Ok(result)=>{
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