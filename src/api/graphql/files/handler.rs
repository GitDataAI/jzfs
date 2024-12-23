use crate::api::app_write::AppWrite;
use crate::api::graphql::files::dto::GraphQLFileDto;
use crate::metadata::service::MetaService;
use actix_web::{web, Responder};

#[utoipa::path(
    post,
    path = "/api/graphql/files",
    request_body = GraphQLFileDto,
    responses(
        (status = 200, description = "Success", body = Vec<u8>),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    ),
    tag = "GraphQL"
)]
pub async fn graphql_files_handler(
    dto: web::Json<GraphQLFileDto>,
    service: web::Data<MetaService>
)
-> impl Responder
{
    let repo = match service.repo_service().owner_name_by_model(dto.owner.clone(), dto.repo.clone()).await{
        Ok(uid) => uid,
        Err(e) => return AppWrite::<Vec<u8>>::fail(e.to_string())
    };
    let tree = match service.repo_service().get_file(repo.uid, dto.branch.clone(), dto.hash.clone(),dto.path.clone()).await{
        Ok(tree) => tree,
        Err(e) => return AppWrite::<Vec<u8>>::fail(e.to_string())
    };
    AppWrite::<Vec<u8>>::ok(tree)
}