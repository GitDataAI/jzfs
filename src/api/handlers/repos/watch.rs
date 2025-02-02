use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use crate::models::users::watchs::Model;
use crate::server::MetaData;
use actix_web::Responder;
use actix_web::web::{Data, Path};

pub async fn repo_watch_count(
    path: Path<(String, String)>,
    meta: Data<MetaData>,
) -> impl Responder {
    match meta.repo_watch_list(path.0.clone(), path.1.clone()).await {
        Ok(items) => AppWrite::<usize>::ok(items.len()),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_watch_list(path: Path<(String, String)>, meta: Data<MetaData>) -> impl Responder {
    match meta.repo_watch_list(path.0.clone(), path.1.clone()).await {
        Ok(items) => AppWrite::<Vec<Model>>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_watch_add(
    path: Path<(String, String, i32)>,
    meta: Data<MetaData>,
    session: actix_session::Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::<Option<String>>::forbidden(e.to_string()),
    };
    let repo = match meta.repo_info(path.0.clone(), path.1.clone()).await {
        Ok(items) => items,
        Err(e) => return AppWrite::<Option<String>>::error(e.to_string()),
    };
    match meta.users_watchs_add(model.uid, repo.uid, path.2).await {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_watch_remove(
    path: Path<(String, String, i32)>,
    meta: Data<MetaData>,
    session: actix_session::Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::<Option<String>>::forbidden(e.to_string()),
    };
    let repo = match meta.repo_info(path.0.clone(), path.1.clone()).await {
        Ok(items) => items,
        Err(e) => return AppWrite::<Option<String>>::error(e.to_string()),
    };
    match meta.users_watchs_remove(model.uid, repo.uid).await {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_watch_update(
    path: Path<(String, String, i32)>,
    meta: Data<MetaData>,
    session: actix_session::Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::<Option<String>>::forbidden(e.to_string()),
    };
    let repo = match meta.repo_info(path.0.clone(), path.1.clone()).await {
        Ok(items) => items,
        Err(e) => return AppWrite::<Option<String>>::error(e.to_string()),
    };
    match meta.users_watchs_update(model.uid, repo.uid, path.2).await {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
