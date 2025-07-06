mod cert;

use crate::endpoint::Endpoint;
use actix_web::web::{post, scope, ServiceConfig};
use crate::routes::cert::{email_captcha, email_verify, users_login, users_logout, users_register};

pub async fn run(endpoint: Endpoint, cfg: &mut ServiceConfig) {
    cfg
        .service(
            scope("/api")
                .service(
                    scope("/auth")
                        .route("/login", post().to(users_login))
                        .route("/register", post().to(users_register))
                        .route("/logout", post().to(users_logout))
                        .route("/email/captcha", post().to(email_captcha))
                        .route("/email/verify", post().to(email_verify))
                )
        )
    ;
}