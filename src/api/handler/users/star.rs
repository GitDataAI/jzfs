use actix_session::Session;
use crate::api::service::Service;
use crate::utils::r::R;
use actix_web::{web, Responder};
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "users",
    path = "/api/v1/users/star/{repo}",
    responses(
            (status = 200, description = "Get Star Success"),
            (status = 400, description = "User Not Exist"),
            (status = 401, description = "Not Login"),
            (status = 402, description = "Repo Not Exist"),
    ),
)]
pub async fn api_user_star_add(
    session: Session,
    repo: web::Path<Uuid>,
    service: web::Data<Service>
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
    match service.users.instar(model.unwrap().uid, repo.into_inner()).await{
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

#[utoipa::path(
    delete,
    tag = "users",
    path = "/api/v1/users/star/{repo}",
    responses(
            (status = 200, description = "Get Star Success"),
            (status = 400, description = "User Not Exist"),
            (status = 401, description = "Not Login"),
            (status = 402, description = "Repo Not Exist"),
    ),
)]
pub async fn api_user_star_remove(
    session: Session,
    repo: web::Path<Uuid>,
    service: web::Data<Service>
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
   match service.users.unstar(model.unwrap().uid, repo.into_inner()).await{
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