use actix_session::Session;
use actix_web::web::{Data, Json, Path};
use crate::api::write::AppWrite;
use crate::model::users::users;
use crate::services::AppState;
use crate::services::product::post::DataProductPostParma;

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
    match status.product_data_post(uid,parma.into_inner(),repo.uid).await {
        Ok(_) => {
            AppWrite::ok(())
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}