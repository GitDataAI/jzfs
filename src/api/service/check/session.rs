use actix_session::Session;
use crate::api::middleware::session::{SessionModel, SESSION_USER_KEY};
use crate::api::service::check::CheckService;

impl CheckService {
    pub async fn check_session(&self, session: Session) -> anyhow::Result<SessionModel>{
        match session.get::<SessionModel>(SESSION_USER_KEY){
            Ok(Some(session)) => Ok(session),
            Ok(None) => Err(anyhow::anyhow!("session not found")),
            Err(e) => Err(anyhow::anyhow!(e))
        }
    }
}