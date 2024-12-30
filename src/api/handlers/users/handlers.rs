use crate::api::app_writer::AppWrite;
use crate::api::handlers::users::options::UserFollowCount;
use crate::api::middleware::session::SessionModel;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{web, Responder};
use std::collections::HashMap;

pub async fn users_info_username(
    path: web::Path<String>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    match meta.users_info_username(path.into_inner()).await {
        Ok(user) => AppWrite::ok(SessionModel::from(&user)),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn user_follower_count_data(
    session: Session,
    meta: web::Data<MetaData>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let uid = if let Some(uid) = query.get("username") {
        match meta.users_info_username(uid.to_string()).await {
            Ok(user) => user.uid,
            Err(err) => return AppWrite::<UserFollowCount>::fail(err.to_string()),
        }
    } else {
        match SessionModel::authenticate(session).await {
            Ok(session) => session.uid,
            Err(err) => return AppWrite::<UserFollowCount>::fail(err.to_string()),
        }
    };
    let follower_count = match meta.users_follower_list(uid).await {
        Ok(count) => count.len(),
        Err(err) => return AppWrite::<UserFollowCount>::fail(err.to_string()),
    };
    let following_count = match meta.users_following_list(uid).await {
        Ok(count) => count.len(),
        Err(err) => return AppWrite::<UserFollowCount>::fail(err.to_string()),
    };
    AppWrite::ok(UserFollowCount {
        follower: follower_count,
        following: following_count,
    })
}
