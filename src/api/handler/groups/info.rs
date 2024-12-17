use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::groups_dto::GroupDesc;
use crate::metadata::service::MetaService;


#[utoipa::path(
    get,
    tag = "groups",
    path = "/api/v1/groups/{group}",
    params(
        ("group" = String, Path, description = "Group Name"),
    ),
    responses(
        (status = 200, description = "Success", body = GroupDesc),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_info(
    service: web::Data<MetaService>,
    group: web::Path<String>
)
 -> impl Responder
{
    let group = service.group_service().info(group.into_inner()).await;
    match group{
        Ok(group)=>{
            AppWrite::ok(GroupDesc::from(&group))
        },
        Err(e)=>{
            AppWrite::fail(e.to_string())
        }
    }
}