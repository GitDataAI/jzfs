use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use crate::models::repos::branchs;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::web::{Data, Path};
use actix_web::Responder;

pub async fn repo_branch_info(
    path: Path<(String, String, String)>,
    meta: Data<MetaData>,
) -> impl Responder {
    match meta
        .repo_branch_info(path.0.clone(), path.1.clone(), path.2.clone())
        .await
    {
        Ok(model) => AppWrite::<branchs::Model>::ok(model),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
pub async fn repo_branch_list(
    path: Path<(String, String)>,
    meta: Data<MetaData>,
) -> impl Responder {
    match meta.repo_branchs(path.0.clone(), path.1.clone()).await {
        Ok(items) => AppWrite::<Vec<branchs::Model>>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_set_default_branch(
    path: Path<(String, String)>,
    meta: Data<MetaData>,
) -> impl Responder {
    match meta
        .repo_default_branch(path.0.clone(), path.1.clone(), "main".to_string())
        .await
    {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_get_default_branch(
    path: Path<(String, String)>,
    meta: Data<MetaData>,
) -> impl Responder {
    match meta
        .repo_default_branch_get(path.0.clone(), path.1.clone())
        .await
    {
        Ok(data) => AppWrite::<branchs::Model>::ok(data),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
pub async fn repo_branch_delete(
    path: Path<(String, String, String)>,
    meta: Data<MetaData>,
    session: Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::unauthorized(e.to_string()),
    };
    let access = match meta
        .repo_groups_teams_access(path.0.clone(), path.1.clone(), model.uid)
        .await
    {
        Ok(access) => access,
        Err(e) => return AppWrite::forbidden(e.to_string()),
    };
    if access < 3 {
        return AppWrite::forbidden("[043] Insufficient Permissions".to_string());
    }
    match meta
        .repo_branch_delete(path.0.clone(), path.1.clone(), path.2.clone())
        .await
    {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_branch_protect(
    path: Path<(String, String, String)>,
    meta: Data<MetaData>,
    session: Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::unauthorized(e.to_string()),
    };
    let access = match meta
        .repo_groups_teams_access(path.0.clone(), path.1.clone(), model.uid)
        .await
    {
        Ok(access) => access,
        Err(e) => return AppWrite::forbidden(e.to_string()),
    };
    if access < 3 {
        return AppWrite::forbidden("[043] Insufficient Permissions".to_string());
    }
    match meta
        .repo_branch_protect(path.0.clone(), path.1.clone(), path.2.clone())
        .await
    {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
pub async fn repo_branch_unprotect(
    path: Path<(String, String, String)>,
    meta: Data<MetaData>,
    session: Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::unauthorized(e.to_string()),
    };
    let access = match meta
        .repo_groups_teams_access(path.0.clone(), path.1.clone(), model.uid)
        .await
    {
        Ok(access) => access,
        Err(e) => return AppWrite::forbidden(e.to_string()),
    };
    if access < 3 {
        return AppWrite::forbidden("[043] Insufficient Permissions".to_string());
    }
    match meta
        .repo_branch_unprotect(path.0.clone(), path.1.clone(), path.2.clone())
        .await
    {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
