use actix_web::web;
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "group",
    path = "/api/v1/group/{group}/repo",
    responses(
        (status = 200, description = "Group found successfully"),
        (status = 400, description = "Group Not Found"),
    ),
)]
pub async fn api_group_repo_get(
    group: web::Path<Uuid>,
    service: web::Data<crate::api::service::Service>,
)
    -> impl actix_web::Responder
{
    let uid = group.into_inner();
    match service.repo.repo_by_group(uid).await{
        Ok(result)=>{
            crate::utils::r::R::<Vec<crate::metadata::model::repos::repo::Model>>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(result),
            }
        },
        Err(e)=>{
            crate::utils::r::R{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}

