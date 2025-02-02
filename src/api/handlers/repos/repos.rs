use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use crate::models::repos::repos;
use crate::models::repos::repos::RepoCreateOptions;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::Responder;
use actix_web::web::{Data, Json, Path};

pub async fn repo_create(
    option: Json<RepoCreateOptions>,
    meta: Data<MetaData>,
    session: Session,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(e) => return AppWrite::<Option<String>>::error(e.to_string()),
    };
    if !option.is_group && option.owner_id != model.uid {
        return AppWrite::<Option<String>>::error(
            "You are not the owner of this group".to_string(),
        );
    }
    if option.is_group {
        let items = match meta.teams_list_by_user(model.uid).await {
            Ok(items) => items,
            Err(e) => return AppWrite::<Option<String>>::error(e.to_string()),
        };
        if !items.iter().any(|x| x.uid == option.owner_id) {
            return AppWrite::<Option<String>>::error(
                "You are not the owner of this group".to_string(),
            );
        }
    }
    match meta.repo_create(option.into_inner()).await {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(e) => AppWrite::<Option<String>>::error(e.to_string()),
    }
}
pub async fn repo_info(path: Path<(String, String)>, meta: Data<MetaData>) -> impl Responder {
    match meta.repo_info(path.0.clone(), path.1.clone()).await {
        Ok(model) => AppWrite::<repos::Model>::ok(model),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_search(meta: Data<MetaData>, option: Path<String>) -> impl Responder {
    match meta.repo_search(option.into_inner()).await {
        Ok(items) => AppWrite::<Vec<repos::Model>>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
