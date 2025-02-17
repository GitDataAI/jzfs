use std::collections::HashMap;
use crate::app::api::write::AppWrite;
use crate::app::services::repo::create::ReposCreateParma;
use crate::app::services::AppState;
use crate::model::users::users;
use poem::session::Session;
use poem::web::{Data, Json, Path};
use poem::{handler, IntoResponse};

#[handler]
pub async fn repo_tree(
    path: Path<(String,String, String,String)>,
    status: Data<&AppState>,
) 
-> impl IntoResponse
{
    let (owner, repo,branch,head) = path.0;
    match status.repo_blob_tree(owner,repo,branch,head).await {
        Ok(tree) => {
            AppWrite::ok(tree)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

#[handler]
pub async fn repo_bhct(
    path: Path<(String,String)>,
    status: Data<&AppState>,
) 
-> impl IntoResponse
{
    let (owner, repo) = path.0;
    match status.repo_blob_bhct(owner,repo).await {
        Ok(bhct) => {
            let bt = bhct.iter().map(|(b,c)|{
                (serde_json::to_string(&b).unwrap(), c.clone())
            })
                .collect::<HashMap<_,_>>();
            AppWrite::ok(bt)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}
#[handler]
pub async fn repo_info(
    path: Path<(String,String)>,
    status: Data<&AppState>,
) 
-> impl IntoResponse
{
    let (owner, repo) = path.0;
    match status.repo_info(owner,repo).await {
        Ok(info) => {
            AppWrite::ok(info)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

#[handler]
pub async fn repo_create(
    session: &Session,
    parma: Json<ReposCreateParma>,
    state: Data<&AppState>,
)
 -> impl IntoResponse
{

    let uid = match session.get::<String>("user"){
        Some(uid) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
            }
        },
        None => {
            return AppWrite::unauthorized("请先登录".to_string())
        }
    };
    match state.repo_create(uid,parma.0).await {
        Ok(_) => {
            AppWrite::ok("创建成功".to_string())
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}