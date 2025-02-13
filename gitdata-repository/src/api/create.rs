use actix_session::Session;
use actix_web::Responder;
use actix_web::web;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::session::UsersSessionModel;
use lib_entity::write::AppWrite;

use crate::service::AppFsState;
use crate::service::create::CreateRepositoryParma;

pub async fn create_repository(
    parma : web::Json<CreateRepositoryParma>,
    service : web::Data<AppFsState>,
    session : Session,
) -> impl Responder {
    let user = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("User Not Login".into()),
    };
    match service.create_repo(user.uid, parma.into_inner()).await {
        Ok(_) => AppWrite::ok(()),
        Err(err) => AppWrite::error(err.to_string()),
    }
}
