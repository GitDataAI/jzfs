use crate::api::app_write::AppWrite;
use crate::api::graphql::tree::dto::GraphQLTreeDto;
use crate::api::handler::check_session;
use crate::metadata::mongo::repotree::RepoTreeModel;
use crate::metadata::service::MetaService;
use actix_session::Session;
use actix_web::{web, Responder};

#[utoipa::path(
    post,
    path = "/api/graphql/tree",
    request_body = GraphQLTreeDto,
    responses(
        (status = 200, description = "Success", body = RepoTreeModel),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    ),
    tag = "GraphQL"
)]
pub async fn graphql_tree_handler(
    session: Session,
    dto: web::Json<GraphQLTreeDto>,
    service: web::Data<MetaService>
)
-> impl Responder
{
    let repo = match service.repo_service().owner_name_by_model(dto.owner.clone(), dto.repo.clone()).await{
        Ok(uid) => uid,
        Err(e) => return AppWrite::<RepoTreeModel>::fail(e.to_string())
    };
    if repo.visible{
        let tree = match service.repo_service().tree(repo.uid, dto.branch.clone(), dto.hash.clone()).await{
            Ok(tree) => tree,
            Err(e) => return AppWrite::<RepoTreeModel>::fail(e.to_string())
        };
        AppWrite::<RepoTreeModel>::ok(tree)
    }else { 
        let model = match check_session(session).await{
            Ok(model) => model,
            Err(e) => return AppWrite::<RepoTreeModel>::fail(e.to_string())
        };
        if model.username == repo.owner{
            let tree = match service.repo_service().tree(repo.uid, dto.branch.clone(), dto.hash.clone()).await{
                Ok(tree) => tree,
                Err(e) => return AppWrite::<RepoTreeModel>::fail(e.to_string())
            };
            return AppWrite::<RepoTreeModel>::ok(tree);
        }
        AppWrite::<RepoTreeModel>::fail("repo not found".to_string())
    }
}