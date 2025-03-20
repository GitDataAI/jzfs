use actix_session::Session;
use uuid::Uuid;

pub async fn switch_context(session: Session, uid: Uuid) -> anyhow::Result<()> {
    match session.get::<Vec<Uuid>>("users_uid") {
        Ok(Some(uids)) => {
            if !uids.contains(&uid) {
                return Err(anyhow::anyhow!("no uid in session"));
            }
        }
        Ok(None) => {
            return Err(anyhow::anyhow!("no uid in session"));
        }
        Err(_) => {
            return Err(anyhow::anyhow!("no uid in session"));
        }
    }
    session.insert("current_uid", uid)?;
    Ok(())
}

pub async fn list_context(session: Session) -> Vec<Uuid> {
    match session.get::<Vec<Uuid>>("users_uid") {
        Ok(Some(uids)) => uids,
        Ok(None) => {
            vec![]
        }
        Err(_) => {
            vec![]
        }
    }
}
