use actix_web::{web, Responder};
use crate::api::service::Service;
use crate::utils::r::R;


#[utoipa::path(
    get,
    tag = "group",
    path = "/api/v1/group/{group}/info",
    params(
        ("group" = Uuid, description = "group Uid"),
    ),
    responses(
        (status = 200, description = "Group found successfully"),
        (status = 400, description = "Group Not Found"),
    ),
)]
pub async fn api_group_info(
    group: web::Path<String>,
    service: web::Data<Service>
)
-> impl Responder
{
    let uid = match uuid::Uuid::parse_str(group.as_str()) {
        Ok(x) => x,
        Err(_) => return R{
            code: 400,
            msg: Option::from("[Error] Invalid UUID".to_string()),
            data: None
        }
    };
    match service.group.info(uid).await{
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