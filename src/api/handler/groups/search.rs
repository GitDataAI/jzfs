use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::groups_dto::{GroupDesc, GroupQuery};
use crate::api::dto::ListOption;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "groups",
    path = "/api/v1/groups",
    params(
        ("key" = String, Query, description = "Search Key"),
    ),
    request_body = ListOption,
    responses(
        (status = 200, description = "Success", body = Vec<GroupDesc>),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_search(
    info: web::Query<GroupQuery>,
    service: web::Data<MetaService>
)
-> impl Responder
{
    let key = info.key.clone();
    match service.group_service().query(key,ListOption{
        page: info.page,
        size: info.size,
    }).await {
        Ok(result) => {
            let result = result
                .iter()
                .map(|x| GroupDesc::from(x))
                .collect::<Vec<_>>();
            AppWrite::<Vec<GroupDesc>>::ok(result)
        }
        Err(e) => {
            AppWrite::error(e.to_string())
        }
    }
}