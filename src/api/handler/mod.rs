use actix_session::Session;
use crate::api::middleware::session::model::{SessionModel, SessionModelKey};

pub mod version;
pub mod health;
pub mod user;
pub mod users;
pub mod groups;
pub mod repos;
pub mod email;

pub async fn check_session(session: Session) -> anyhow::Result<SessionModel>{
    match session.get::<SessionModel>(SessionModelKey){
        Ok(Some(session)) => Ok(session),
        Ok(None) => Err(anyhow::anyhow!("session not found")),
        Err(e) => Err(anyhow::anyhow!(e))
    }
}