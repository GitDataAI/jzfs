use crate::api::service::check::CheckService;
use crate::api::service::email::EmailService;
use crate::api::service::groups::GroupService;
use crate::api::service::repos::RepoService;
use crate::api::service::teams::TeamService;
use crate::api::service::users::UserService;
use crate::metadata::transaction::repos::RepoTransaction;
use crate::server::db::DB;
use crate::server::email::EmailServer;

pub mod users;
pub mod repos;
pub mod teams;
pub mod groups;
pub mod email;
pub mod check;

#[derive(Clone)]
pub struct Service{
    pub email: EmailService,
    pub users: UserService,
    pub group: GroupService,
    pub check: CheckService,
    pub team: TeamService,
    pub repo: RepoService,
}

impl Service {
    pub async fn new() -> Service {
        let db = DB.get().unwrap();
        let email = EmailServer::init().await;
        Self{
            email: EmailService{
                server: email,
            },
            users: UserService{
                db: db.clone(),
            },
            group: GroupService{
                db: db.clone(),
            },
            check: CheckService{
                db: db.clone(),
            },
            team: TeamService{
                db: db.clone(),
            },
            repo: RepoService{
                db: db.clone(),
                transaction: RepoTransaction::new(db.clone()),
            }
        }
    }
}