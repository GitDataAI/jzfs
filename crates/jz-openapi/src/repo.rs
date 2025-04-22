use actix_web::web::Data;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jz_module::AppModule;
use jz_module::repo::list::RepositoryListParam;

pub async fn repo_list(
    service: Data<AppModule>,
    query: actix_web::web::Query<RepositoryListParam>,
    credentials: Option<BearerAuth>,
)
    -> actix_web::Result<actix_web::HttpResponse>
{
    let Some(credentials) = credentials else {
        return Ok(actix_web::HttpResponse::Unauthorized().body("token invalid"))
    };
    let Ok(token) = service.token_find(credentials.token().to_string()).await else {
        return Ok(actix_web::HttpResponse::Unauthorized().body("token invalid"))
    };
    let _access = token.access;
    let result = service.repo_list(query.into_inner()).await;
    match result {
        Ok(data) => {
            Ok(actix_web::HttpResponse::Ok().json(data))
        }
        Err(err) => {
            Ok(actix_web::HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}