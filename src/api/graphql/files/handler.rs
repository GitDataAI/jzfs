use crate::api::app_write::AppWrite;
use crate::api::graphql::files::dto::{GraphQLFileDto, GraphQLFileOV};
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
        Err(e) => return AppWrite::<GraphQLFileOV>::fail(e.to_string())
    };
    let tree = match service.repo_service().get_file(repo.uid, dto.branch.clone(), dto.hash.clone(),dto.path.clone()).await{
        Ok(tree) => tree,
        Err(e) => return AppWrite::<GraphQLFileOV>::fail(e.to_string())
    };
    if dto.block < 0{
        AppWrite::<GraphQLFileOV>::ok(GraphQLFileOV{
            total: 1,
            current: dto.block,
            size: dto.size_limit,
            data: tree,
        })
    }else {
        let size = (dto.block * dto.size_limit);
        let next_size = ((dto.block + 1) * dto.size_limit);
        if size > tree.len() as i32{
            return AppWrite::<GraphQLFileOV>::ok(GraphQLFileOV{
                total: tree.len() as i32 / dto.size_limit,
                current: dto.block,
                size: dto.size_limit,
                data: tree
            })
        }else if next_size > tree.len() as i32 && tree.len() as i32 > size{
            return AppWrite::<GraphQLFileOV>::ok(GraphQLFileOV{
                total: tree.len() as i32 / dto.size_limit,
                current: dto.block,
                size: dto.size_limit,
                data: tree[(dto.block * dto.size_limit) as usize..tree.len()].to_vec(),
            })
        }else {
            let result = tree[(dto.block * dto.size_limit) as usize..((dto.block + 1) * dto.size_limit) as usize].to_vec();
            AppWrite::<GraphQLFileOV>::ok(GraphQLFileOV{
                total: tree.len() as i32 / dto.size_limit,
                current: dto.block,
                size: dto.size_limit,
                data: result,
            })
        }
    }
 
}

