use crate::auth::user_context::api_auth_user_context;
use crate::auth::user_login::api_auth_user_login;
use crate::auth::user_logout::api_auth_user_logout;
use crate::auth::user_register::{
    api_auth_user_register, api_auth_user_register_after, api_auth_user_register_after_captcha,
    api_auth_user_register_after_captcha_verify,
};
use crate::repos::commits::api_repos_commit_list;
use crate::repos::data::api_repo_data;
use crate::repos::init::{
    api_repo_init, api_repo_init_before, api_repo_init_owner_select, api_repo_init_storage,
};
use crate::repos::recommend::api_repos_recommend;
use crate::repos::refs::{api_repos_refs_delete, api_repos_refs_list};
use crate::repos::star::{api_repos_star_repo, api_repos_unstar_repo};
use crate::repos::tree::api_repos_tree;
use crate::repos::watch::{api_repos_unwatch_repo, api_repos_watch_repo};
use crate::user::settings::access_key::{
    api_user_setting_access_key_delete, api_user_setting_access_key_insert,
    api_user_setting_access_key_list,
};
use crate::user::settings::avatar::api_setting_avatar_upload;
use crate::user::settings::basic::{api_setting_basic, api_setting_basic_get};
use crate::user::settings::ssh_key::{
    api_user_setting_ssh_key_delete, api_user_setting_ssh_key_insert, api_user_setting_ssh_key_list,
};
use crate::users::actives::api_users_active;
use crate::users::users::{api_users, api_users_repos, api_users_star, api_users_watch};
use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use actix_web::web::scope;
use actix_web::{HttpServer, middleware, web};
use actix_web::guard::GuardContext;
use config::AppConfig;
pub use core::*;
use error::AppError;
use session::config::{PersistentSession, SessionLifecycle, TtlExtensionPolicy};
use session::{SessionMiddleware, SessionStorage};
use sha2::{Digest, Sha512};
use tracing::info;
use git::transport::http::info::git_refs;
use git::transport::http::receive_pack::git_receive_pack;
use git::transport::http::upload_pack::git_upload_pack;
use crate::guard::git::git_guard;

pub type AppStatus = web::Data<AppCore>;

#[derive(Clone, Debug)]
pub struct AppApiService {
    pub core: AppCore,
    pub session: SessionStorage,
    pub config: AppConfig,
}

impl AppApiService {
    pub async fn run(self) -> Result<(), AppError> {
        let core = self.core.clone();
        let db = self.core.db.clone();
        let redis = self.core.redis.clone();
        info!("Starting API service...");
        info!(
            "Starting server at {}:{}",
            self.config.api.host, self.config.api.port
        );
        info!("Initializing session middleware");
        let storage = SessionStorage::new(db.clone(), redis.clone());
        let key = self.config.api.session.to_string();
        let mut hasher = Sha512::new();
        hasher.update(key.as_bytes());
        let key_bytes = hasher.finalize().as_slice().to_vec();
        let mut key = key_bytes;
        if key.len() != 64 {
            tracing::warn!("session key length is not 64, use random key");
            key = [0; 64].to_vec();
        }
        let cfg = self.config.clone();
        HttpServer::new(move || {
            actix_web::App::new()
                .app_data(web::Data::new(storage.clone()))
                .app_data(web::Data::new(core.clone()))
                .app_data(web::Data::new(db.clone()))
                .app_data(web::Data::new(redis.clone()))
                .app_data(web::Data::new(cfg.clone()))
                .wrap(
                    SessionMiddleware::builder(storage.clone(), Key::from(&key.clone()))
                        .session_lifecycle(SessionLifecycle::PersistentSession(
                            PersistentSession::default()
                                .session_ttl(Duration::days(self.config.api.max_age))
                                .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                        ))
                        .cookie_name(self.config.api.session.clone())
                        .cookie_domain(None)
                        .cookie_secure(false)
                        .cookie_path("/".to_string())
                        .build(),
                )
                .wrap(middleware::Logger::default())
                .wrap(middleware::Compress::default())
                .wrap(middleware::Identity::default())
                .configure(|configure| Self::configure(configure))
        })
        .workers(self.config.api.workers)
        .bind(format!("{}:{}", self.config.api.host, self.config.api.port))?
        .run()
        .await?;
        Ok(())
    }
    pub fn configure(app: &mut web::ServiceConfig) {
        app.service(
            scope("/api")
                .service(
                    scope("/auth")
                        .route("/context", web::post().to(api_auth_user_context))
                        .route("/login", web::post().to(api_auth_user_login))
                        .route("/register", web::post().to(api_auth_user_register))
                        .route(
                            "/register/after",
                            web::post().to(api_auth_user_register_after),
                        )
                        .route(
                            "/register/send",
                            web::post().to(api_auth_user_register_after_captcha),
                        )
                        .route(
                            "/register/verify",
                            web::post().to(api_auth_user_register_after_captcha_verify),
                        )
                        .route("/logout", web::post().to(api_auth_user_logout)),
                )
                .service(
                    scope("/user").service(
                        scope("/setting")
                            .route("/basic", web::get().to(api_setting_basic_get))
                            .route("/basic", web::post().to(api_setting_basic))
                            .route("/avatar", web::post().to(api_setting_avatar_upload))
                            .service(
                                scope("/ssh-key")
                                    .route("", web::get().to(api_user_setting_ssh_key_list))
                                    .route("", web::post().to(api_user_setting_ssh_key_insert))
                                    .route(
                                        "/{name}",
                                        web::delete().to(api_user_setting_ssh_key_delete),
                                    ),
                            )
                            .service(
                                scope("/access-key")
                                    .route("", web::get().to(api_user_setting_access_key_list))
                                    .route("", web::post().to(api_user_setting_access_key_insert))
                                    .route(
                                        "/{name}",
                                        web::delete().to(api_user_setting_access_key_delete),
                                    ),
                            ),
                    ),
                )
                .service(
                    scope("/users").service(
                        scope("/{username}")
                            .route("", web::get().to(api_users))
                            .route("/active", web::get().to(api_users_active))
                            .route("/star", web::get().to(api_users_star))
                            .route("/watch", web::get().to(api_users_watch))
                            .route("/repo", web::get().to(api_users_repos)),
                    ),
                )
                .service(
                    scope("/repo")
                        .route("", web::get().to(api_repos_recommend))
                        .service(
                            scope("/init")
                                .route("", web::post().to(api_repo_init))
                                .route("", web::patch().to(api_repo_init_before))
                                .route("/owner", web::get().to(api_repo_init_owner_select))
                                .route("/storage", web::get().to(api_repo_init_storage)),
                        )
                        .service(
                            scope("/{owner}/{repo}")
                                .route("", web::get().to(api_repo_data))
                                .service(
                                    scope("/refs")
                                        .route("", web::get().to(api_repos_refs_list))
                                        .route(
                                            "/{ref_name}",
                                            web::delete().to(api_repos_refs_delete),
                                        ),
                                )
                                .service(
                                    scope("/commit/{ref_name}")
                                        .route("", web::get().to(api_repos_commit_list)),
                                )
                                .service(
                                    scope("/tree").route(
                                        "{ref_name}/{path:.*}",
                                        web::get().to(api_repos_tree),
                                    ),
                                )
                                .service(
                                    scope("/star")
                                        .route("", web::post().to(api_repos_star_repo))
                                        .route("", web::delete().to(api_repos_unstar_repo)),
                                )
                                .service(
                                    scope("/watch")
                                        .route("", web::post().to(api_repos_watch_repo))
                                        .route("", web::delete().to(api_repos_unwatch_repo)),
                                ),
                        ),
                ),
        )
        .service(
            scope("")
                .guard(|context:&GuardContext| {
                    git_guard(context)
                })
                .service(
                    scope("/{owner}/{repo}")
                        .route("/git-upload-pack", web::post().to(git_upload_pack))
                        .route("/git-receive-pack", web::post().to(git_receive_pack))
                        .route("/info/refs", web::get().to(git_refs))
                )
        )
        .service(web_ui::web_ui)
        ;
    }
}

pub mod auth;
pub mod repos;
pub mod user;
pub mod users;
pub mod guard;