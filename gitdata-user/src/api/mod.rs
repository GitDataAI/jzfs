use actix_session::Session;
use actix_web::Responder;
use actix_web::web;
use actix_web::web::scope;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::session::UsersSessionModel;
use lib_entity::write::AppWrite;

use crate::server::AppUserState;

pub mod email;
pub mod follow;
pub mod optional;
pub mod ssh_key;
pub mod token;
pub fn router(cfg : &mut web::ServiceConfig) {
    cfg.service(
        scope("/user")
            .route("/index", web::patch().to(index))
            .service(
                scope("/email")
                    .route("/can", web::get().to(email::main_email_can))
                    .route("/cannot", web::get().to(email::main_email_cannot))
                    .route("/is", web::get().to(email::main_email_is))
                    .route("/update", web::post().to(email::main_email_update))
                    .route("/add", web::post().to(email::main_email_add))
                    .route("/delete", web::post().to(email::main_email_delete))
                    .route("/list", web::get().to(email::main_email_list))
                    .route(
                        "/set_primary",
                        web::post().to(email::main_email_set_primary),
                    )
                    .route(
                        "/set_no_primary",
                        web::post().to(email::main_email_set_no_primary),
                    ),
            )
            .service(
                scope("/optional")
                    .route("/update", web::post().to(optional::update_optional))
                    .route("/acquire", web::get().to(optional::acquire_optional)),
            )
            .service(
                scope("/follow")
                    .route("/add", web::post().to(follow::users_follow))
                    .route("/list", web::get().to(follow::users_follow_list))
                    .route("/delete", web::post().to(follow::users_unfollow)),
            )
            .service(
                scope("/token")
                    .route("/create", web::post().to(token::create_token))
                    .route("/list", web::get().to(token::list_token))
                    .route("/delete", web::post().to(token::delete_token)),
            )
            .service(
                scope("/ssh_key")
                    .route("/create", web::post().to(ssh_key::user_add_ssh_key))
                    .route("/list", web::get().to(ssh_key::user_list_ssh_key))
                    .route("/delete", web::post().to(ssh_key::user_delete_ssh_key)),
            ),
    );
}

async fn index(session : Session, state : web::Data<AppUserState>) -> impl Responder {
    let user = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("Not Login".to_string()),
    };
    match state.get_user_info(user.uid).await {
        Ok(info) => AppWrite::ok(info),
        Err(err) => AppWrite::error(err.to_string()),
    }
}
