use sea_orm::DatabaseConnection;
use crate::metadata::transaction::users::UserTransaction;

pub mod login;
pub mod apply;
pub mod check;
pub mod reset;
pub mod email;
pub mod following;
pub mod info;
pub mod setting;
pub mod key;
pub mod avatar;
// pub mod update;
// pub mod info;
#[derive(Clone)]
pub struct UserService{
    pub(crate) db: DatabaseConnection,
    pub(crate) txn: UserTransaction,
}

