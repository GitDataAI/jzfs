use crate::ROOT_PATH;
use crate::emails::Email;
use crate::git::GitServer;
use crate::utils::db::{EMAIL_SERVICE, Postgres};
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use tokio::sync::OnceCell;

pub mod check;
pub mod emails;
pub mod repos;
pub mod teams;
pub mod users;
#[derive(Clone)]
pub struct MetaData {
    database: DatabaseConnection,
    email: Email,
    git: GitServer,
}
pub static META: OnceCell<MetaData> = OnceCell::const_new();

impl MetaData {
    pub async fn init() -> Self {
        let email = EMAIL_SERVICE.get().unwrap().clone();
        let git = GitServer {
            root: PathBuf::from(ROOT_PATH),
        };
        let once = Self {
            database: Postgres().await,
            email: email.clone(),
            git,
        };
        let _ = META.get_or_init(|| async { once.clone() }).await.clone();
        once
    }
}
