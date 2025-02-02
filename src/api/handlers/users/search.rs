use crate::api::app_writer::AppWrite;
use crate::api::handlers::users::options::{Base64Inner, PageParma};
use crate::api::middleware::session::SessionModel;
use crate::server::MetaData;
use actix_web::{Responder, web};

pub async fn users_search(
    inner: web::Json<Base64Inner>,
    meta: web::Data<MetaData>,
    query: web::Query<PageParma>,
) -> impl Responder {
    let inner = match inner.decode::<String>() {
        Ok(inner) => inner,
        Err(err) => return AppWrite::<Vec<SessionModel>>::fail(err.to_string()),
    };
    match meta.users_search(inner, query.page, query.size).await {
        Ok(users) => AppWrite::ok(
            users
                .iter()
                .map(|x| SessionModel::from(x))
                .collect::<Vec<_>>(),
        ),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
