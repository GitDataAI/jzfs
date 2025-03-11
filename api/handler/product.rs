use crate::api::write::AppWrite;
use crate::model::users::users;
use crate::services::product::post::{DataProductPost, DataProductPostParma};
use crate::services::AppState;
use actix_files::NamedFile;
use actix_session::Session;
use actix_web::web::{Data, Json, Path};
use uuid::Uuid;

pub async fn product_post(
    session: Session,
    parma: Json<DataProductPostParma>,
    status: Data<AppState>,
    path: Path<(String,String)>
)
 -> impl actix_web::Responder
{
    let (owner, repo) = path.into_inner();
    let uid = match session.get::<String>("user"){
        Ok(Some(uid)) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::<()>::unauthorized("请先登录".to_string())
            }
        },
        Ok(None) => {
            return AppWrite::unauthorized("请先登录".to_string())
        },
        Err(_) => {
            return AppWrite::unauthorized("请先登录".to_string())
        }
    };
    let repo = match status.repo_info(owner,repo).await {
        Ok(info) => {
            info
        },
        Err(err) => {
            return AppWrite::error(err.to_string())
        }
    };
    match DataProductPost::send(
        uid,
        parma.into_inner(),
        repo.uid
    ).await {
        Ok(_) => {
            AppWrite::ok(())
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn product_list(
    parma: Json<crate::services::product::list::ProductListParam>,
    status: Data<AppState>,
)
 -> impl actix_web::Responder
{
    match status.product_list(parma.into_inner()).await {
        Ok(list) => {
            AppWrite::ok(list)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn product_info(
    path: Path<Uuid>,
    status: Data<AppState>,
)
 -> impl actix_web::Responder
{
    let uuid = path.into_inner();
    match status.product_info(uuid).await {
        Ok(info) => {
            AppWrite::ok(info)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn product_download(
    path: Path<Uuid>,
    status: Data<AppState>,
)
 -> impl actix_web::Responder
{
    let uuid = path.into_inner();
    match status.product_data_download_zip(uuid).await {
        Ok(info) => {
            info
        },
        Err(_) => {
            NamedFile::open("static/error.html").unwrap()
        }
    }
}