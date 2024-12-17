use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::groups_dto::GroupDesc;
use crate::api::dto::user_dto::UserOv;
use crate::api::handler::check_session;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "groups",
    path = "/api/v1/groups/{group}/members",
    params(
        ("group" = String, Path, description = "Group Name"),
    ),
    responses(
        (status = 200, description = "Success", body = Vec<UserOv>),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_members(
    service: web::Data<MetaService>,
    group: web::Path<String>
) -> impl Responder
{
    let groups = service.group_service().name_to_uid(group.into_inner()).await;
    if groups.is_err(){
        return AppWrite::fail("Not Found".to_string());
    }
    let members = service.group_service().members(groups.unwrap()).await;
    match members{
        Ok(members) => {
            AppWrite::ok(members)
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    put,
    tag = "groups",
    path = "/api/v1/groups/{group}/members/{user}",
    params(
        ("group" = String, Path, description = "Group Name"),
        ("user" = String, Path, description = "User Name"),
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_member_add(
    service: web::Data<MetaService>,
    path: web::Path<(String,String)>,
) -> impl Responder
{
    let (group,user) = path.into_inner();
    let groups = service.group_service().name_to_uid(group).await;
    if groups.is_err(){
        return AppWrite::fail("Not Found".to_string());
    }
    let users = service.user_service().username_to_uid(user).await;
    if users.is_err(){
        return AppWrite::fail("Not Found".to_string());
    }
    let members = service.group_service().member_add(groups.unwrap(),users.unwrap(),2).await;
    match members{
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
    tag = "groups",
    path = "/api/v1/groups/{group}/members/{user}",
    params(
        ("group" = String, Path, description = "Group Name"),
        ("user" = String, Path, description = "User Name"),
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_member_remove(
    service: web::Data<MetaService>,
    path: web::Path<(String,String)>,
) -> impl Responder
{
    let (group,user) = path.into_inner();
    let groups = service.group_service().name_to_uid(group).await;
    if groups.is_err(){
        return AppWrite::fail("Not Found".to_string());
    }
    let users = service.user_service().username_to_uid(user).await;
    if users.is_err(){
        return AppWrite::fail("Not Found".to_string());
    }
    let members = service.group_service().member_remove(groups.unwrap(),users.unwrap()).await;
    match members{
        Ok(_) => {
            AppWrite::ok("Success".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}
#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/groups",
    responses(
        (status = 200, description = "Success", body = Vec<GroupDesc>),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_user_groups(
    service: web::Data<MetaService>,
    session: Session
) -> impl Responder
{
    let uid = match check_session(session).await{
        Ok(uid) => uid.uid,
        Err(_) => return AppWrite::fail("Not Login".to_string())
    };
    let groups = service.group_service().find_member(uid).await;
    match groups{
        Ok(groups) => {
            AppWrite::ok(groups.iter().map(|x| GroupDesc::from(x)).collect::<Vec<_>>())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}