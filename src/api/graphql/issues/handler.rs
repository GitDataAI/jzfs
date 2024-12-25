use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::graphql::issues::dto::GraphQLRepoIssuesDto;
use crate::metadata::mongo::issues::IssuesModel;
use crate::metadata::service::MetaService;


pub async fn graphql_issues_handler(
    dto: web::Json<GraphQLRepoIssuesDto>,
    service: web::Data<MetaService>
)
-> impl Responder
{
    if dto.issues_id.is_none(){
        match service.repo_service().issues(dto.owner.clone(), dto.repo.clone(), dto.page.clone(), dto.size.clone()).await {
            Ok(issues) => {
                AppWrite::<Vec<IssuesModel>>::ok(issues)
            },
            Err(e) => {
                AppWrite::error(e.to_string())
            }
        }
    }else { 
        match service.repo_service().issues_detail(dto.issues_id.unwrap()).await {
            Ok(issues) => {
                AppWrite::<Vec<IssuesModel>>::ok(vec![issues])
            },
            Err(e) => {
                AppWrite::error(e.to_string())
            }
        }
    }
}