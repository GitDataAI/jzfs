use crate::app::api::write::AppWrite;
use crate::app::services::repo::blob::RepoBlobFile;
use crate::app::services::repo::create::ReposCreateParma;
use crate::app::services::repo::fork::ForkParma;
use crate::app::services::statistics::repo::CLICK;
use crate::app::services::AppState;
use crate::model::users::users;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::web::{Json, Path};
use actix_web::{HttpResponseBuilder, Responder};
use std::collections::HashMap;

pub async fn repo_tree(
    path: Path<(String,String, String,String)>,
    status: Data<AppState>,
) 
-> impl Responder
{
    let (owner, repo,branch,head) = path.into_inner();
    match status.repo_blob_tree(owner,repo,branch,head).await {
        Ok(tree) => {
            AppWrite::ok(tree)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}


pub async fn repo_bhct(
    path: Path<(String,String)>,
    status: Data<AppState>,
) 
-> impl Responder
{
    let (owner, repo) = path.into_inner();
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
pub async fn repo_info(
    path: Path<(String,String)>,
    status: Data<AppState>,
)
    -> impl Responder
{
    let (owner, repo) = path.into_inner();
    match status.repo_info(owner,repo).await {
        Ok(info) => {
            status.statistics_repo(info.uid,CLICK.to_string()).await.ok();
            AppWrite::ok(info)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn repo_create(
    session: Session,
    parma: Json<ReposCreateParma>,
    state: Data<AppState>,
)
 -> impl Responder
{

    let uid = match session.get::<String>("user"){
       Ok(Some(uid)) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
            }
        },
        Ok(None) => {
            return AppWrite::unauthorized("请先登录".to_string())
        }
        Err(_) => {
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

pub async fn repo_file(
    param: Json<RepoBlobFile>,
    status: Data<AppState>,
) 
-> impl Responder
{
    match status.repo_blob_file(param.0).await {
        Ok(file) => {
            HttpResponseBuilder::new(StatusCode::OK)
                .content_type("application/octet-stream")
                .body(file)
        },
        Err(err) => {
            HttpResponseBuilder::new(StatusCode::NOT_FOUND)
                .body(err.to_string())
        }
    }
}

pub async fn repo_fork(
    session: Session,
    path: Path<(String,String)>,
    parma: Json<ForkParma>,
    status: Data<AppState>,
) 
 -> impl Responder
{
    let (owner, repo) = path.into_inner();
    let uid = match session.get::<String>("user"){
        Ok(Some(uid)) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
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
    match status.repo_fork(uid,repo.uid,parma.0).await {
        Ok(_) => {
            AppWrite::ok("创建成功".to_string())
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn repo_star(
    session: Session,
    path: Path<(String,String)>,
    status: Data<AppState>,
) 
-> impl Responder
{
    let (owner, repo) = path.into_inner();
    let uid = match session.get::<String>("user"){
        Ok(Some(uid)) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
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
    match status.repo_star(uid,repo.uid).await {
        Ok(_) => {
            AppWrite::ok("创建成功".to_string())
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}
pub async fn repo_watch(
    session: Session,
    path: Path<(String,String, i32)>,
    status: Data<AppState>,
) 
-> impl Responder
{
    let (owner, repo,level) = path.into_inner();
    let uid = match session.get::<String>("user"){
        Ok(Some(uid)) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
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
    match status.repo_watch(uid,repo.uid,level).await {
        Ok(_) => {
            AppWrite::ok("创建成功".to_string())
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn repo_access(
    session: Session,
    status: Data<AppState>,
)
 -> impl Responder
{
    let uid = match session.get::<String>("user"){
        Ok(Some(uid)) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
            }
        },
        Ok(None) => {
            return AppWrite::unauthorized("请先登录".to_string())
        },
        Err(_) => {
            return AppWrite::unauthorized("请先登录".to_string())
        }
    };
    let access = match status.user_access_owner(uid).await {
        Ok(access) => {
            access
        },
        Err(err) => {
            return AppWrite::error(err.to_string())
        }
    };
    AppWrite::ok(access)
}

pub async fn repo_commit_one(
    path: Path<(String,String,String,String)>,
    status: Data<AppState>,
) 
-> impl Responder
{
    let (owner, repo, branch, sha) = path.into_inner();
    match status.repo_commit_one(owner,repo,branch,sha).await {
        Ok(commit) => {
            AppWrite::ok(commit)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}