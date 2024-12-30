use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use crate::models::users::star::Model;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::web::{Data, Path};
use actix_web::Responder;

pub async fn repo_star_count(path: Path<(String, String)>, meta: Data<MetaData>) -> impl Responder {
    match meta.repo_info(path.0.clone(), path.1.clone()).await {
        Ok(items) => AppWrite::<i64>::ok(items.nums_star),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_star_list(path: Path<(String, String)>, meta: Data<MetaData>) -> impl Responder {
    match meta.repo_star_list(path.0.clone(), path.1.clone()).await {
        Ok(items) => AppWrite::<Vec<Model>>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_star_add(
    path: Path<(String, String)>,
    meta: Data<MetaData>,
    session: Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::<Option<String>>::forbidden(e.to_string()),
    };
    let repo = match meta.repo_info(path.0.clone(), path.1.clone()).await {
        Ok(items) => items,
        Err(e) => return AppWrite::<Option<String>>::error(e.to_string()),
    };
    match meta.users_star_add(model.uid, repo.uid).await {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
pub async fn repo_star_remove(
    path: Path<(String, String)>,
    meta: Data<MetaData>,
    session: Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::<Option<String>>::forbidden(e.to_string()),
    };
    let repo = match meta.repo_info(path.0.clone(), path.1.clone()).await {
        Ok(items) => items,
        Err(e) => return AppWrite::<Option<String>>::error(e.to_string()),
    };
    match meta.users_star_del(model.uid, repo.uid).await {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
pub async fn repo_owner_star_list(meta: Data<MetaData>, session: Session) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::<Vec<Model>>::forbidden(e.to_string()),
    };
    match meta.users_star_list(model.uid).await {
        Ok(items) => AppWrite::<Vec<Model>>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
