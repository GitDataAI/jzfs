use crate::api::app_writer::AppWrite;
use crate::api::handlers::users::options::UserKeyCreate;
use crate::api::middleware::session::SessionModel;
use crate::models::users::ssh_key::Model;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{Responder, web};
use uuid::Uuid;

pub async fn users_key_add(
    session: Session,
    inner: web::Json<UserKeyCreate>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Option<String>>::fail(err.to_string()),
    };
    let ssh = inner.pubkey.clone();
    let name = inner.pubkey.clone();
    let access = inner.access;
    let expire = inner.expire;
    match meta
        .users_ssh_add(session.uid, ssh, access, expire, name)
        .await
    {
        Ok(_unused) => AppWrite::ok(None),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
pub async fn users_key_del(
    session: Session,
    path: web::Path<Uuid>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Option<String>>::fail(err.to_string()),
    };
    match meta.users_ssh_del(session.uid, path.into_inner()).await {
        Ok(_unused) => AppWrite::ok(None),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn users_key_get_by_uid(
    session: Session,
    path: web::Path<Uuid>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Model>::fail(err.to_string()),
    };
    let uid = path.into_inner();
    match meta.users_ssh_list(session.uid).await {
        Ok(ssh) => {
            if let Some(ssh) = ssh.iter().find(|x| x.uid == uid) {
                AppWrite::ok(ssh.clone())
            } else {
                AppWrite::fail("not found".to_string())
            }
        }
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn users_key_get(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Vec<Model>>::fail(err.to_string()),
    };
    match meta.users_ssh_list(session.uid).await {
        Ok(ssh) => AppWrite::ok(
            ssh.iter()
                .map(|x| {
                    let mut x = x.clone();
                    x.content = String::new();
                    x
                })
                .collect::<Vec<_>>(),
        ),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
