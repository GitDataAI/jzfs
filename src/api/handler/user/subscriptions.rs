use actix_session::Session;
use actix_web::web;
use crate::api::app_write::AppWrite;
use crate::api::dto::groups_dto::GroupDesc;
use crate::api::handler::check_session;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/subscriptions",
    responses(
        (status = 200, description = "Success", body = Vec<GroupDesc>),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_user_subscriptions(
    session: Session,
    service: web::Data<MetaService>
)
-> impl actix_web::Responder
{
    let uid = match check_session(session).await{
        Ok(uid) => uid.uid,
        Err(_) => return AppWrite::fail("Not Login".to_string())
    };
    let groups = service.group_service().find_member(uid).await;
    match groups{
        Ok(groups) => {
            AppWrite::ok(groups.iter().map(|x| GroupDesc::from(x)).collect::<Vec<_>>())
        }
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}


#[utoipa::path(
    put,
    tag = "user",
    path = "/api/v1/user/subscriptions/{owner}/{repo}",
    params(
        ("owner" = String, Path, description = "Group Owner"),
        ("repo" = String, Path, description = "Group Name"),
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_user_subscription_add(
    service: web::Data<MetaService>,
    path: web::Path<(String,String)>,
    session: Session,
) -> impl actix_web::Responder
{
    let uid = match check_session(session).await{
        Ok(uid) => uid.uid,
        Err(_) => return AppWrite::fail("Not Login".to_string())
    };
    let (owner, repo) = path.into_inner();
    let repos = service.repo_service().owner_name_by_uid(owner,repo).await;
    if repos.is_err(){
        return AppWrite::fail("Not Found".to_string());
    }
    let groups = service.user_service().wacthher_add(uid, repos.unwrap()).await;
    match groups{
        Ok(_) => {
            AppWrite::ok("Success".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    delete,
    tag = "user",
    path = "/api/v1/user/subscriptions/{owner}/{repo}",
    params(
        ("owner" = String, Path, description = "Group Owner"),
        ("repo" = String, Path, description = "Group Name"),
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_user_subscription_remove(
    service: web::Data<MetaService>,
    path: web::Path<(String,String)>,
    session: Session,
) -> impl actix_web::Responder
{
    let uid = match check_session(session).await{
        Ok(uid) => uid.uid,
        Err(_) => return AppWrite::fail("Not Login".to_string())
    };
    let (owner, repo) = path.into_inner();
    let repos = service.repo_service().owner_name_by_uid(owner,repo).await;
    if repos.is_err(){
        return AppWrite::fail("Not Found".to_string());
    }
    let groups = service.user_service().wacthher_remove(uid, repos.unwrap()).await;
    match groups{
        Ok(_) => {
            AppWrite::ok("Success".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}