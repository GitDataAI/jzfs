use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::handler::check_session;
use crate::metadata::model::repo::repo;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/staring",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_staring(
    session: Session,
    service: web::Data<MetaService>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<Vec<repo::Model>>::unauthorized(model.err().unwrap().to_string()) 
    }
    let model = model.unwrap();
    match service.user_service().star(model.uid).await{
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
    path = "/api/v1/user/star/{owner}/{name}",
    responses(
        (status = 200, description = "Success", body = String),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_star_add(
    session: Session,
    query: web::Path<(String, String)>,
    service: web::Data<MetaService>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<String>::unauthorized(model.err().unwrap().to_string())
    }
    let repo_id = match service.repo_service().owner_name_by_uid(query.0.clone(), query.1.clone()).await{
            Ok(result)=> result,
            Err(e)=>{
                return AppWrite::fail(e.to_string())
            }
    };
    match service.user_service().instar(model.unwrap().uid, repo_id).await{
        Ok(_) => {
            AppWrite::ok_msg(String::from("Ok"))
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/unstar/{owner}/{name}",
    responses(
        (status = 200, description = "Success", body = String),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_star_remove(
    session: Session,
    query: web::Path<(String, String)>,
    service: web::Data<MetaService>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<String>::unauthorized(model.err().unwrap().to_string())
    }
    let repo_id = match service.repo_service().owner_name_by_uid(query.0.clone(), query.1.clone()).await{
        Ok(result)=> result,
        Err(e)=>{
            return AppWrite::fail(e.to_string())
        }
    };
    match service.user_service().unstar(model.unwrap().uid, repo_id).await{
        Ok(_) => {
            AppWrite::ok_msg(String::from("Ok"))
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}