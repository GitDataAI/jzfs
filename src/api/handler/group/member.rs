use actix_web::{web, Responder};
use uuid::Uuid;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "group",
    path = "/api/v1/group/{group}/member",
    responses(
        (status = 200, description = "Group found successfully"),
        (status = 400, description = "Group Not Found"),
    ),
)]
pub async fn api_group_member(
    group: web::Path<Uuid>,
    service: web::Data<Service>
)
-> impl Responder
{
    match service.group.member(group.into_inner()).await{
        Ok(x) => R{
            code: 200,
            msg: Option::from("[Ok]".to_string()),
            data: Some(x)
        },
        Err(_) => R{
            code: 400,
            msg: Option::from("[Error] Group Not Found".to_string()),
            data: None
        }
    }
}
