use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path};
use jz_module::AppModule;
use jz_module::repo::file::blob::BlobQuery;
use crate::utils::request::RequestBody;

pub async fn repo_blob(
    module: Data<AppModule>,
    path: Path<(String, String)>,
    payload: RequestBody<BlobQuery>,
) -> impl Responder {
    let (owner,repo) = path.into_inner();
    match module
        .repo_blob(
            owner,
            repo,
            payload.into_inner().inner,
        )
        .await
    {
        Ok(e) => {
            HttpResponse::Ok().body(e)
        }
        Err(e) => {
            HttpResponse::NotFound().body(e.to_string())
        }
    }
}
