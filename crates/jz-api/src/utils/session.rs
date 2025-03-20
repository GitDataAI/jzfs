use actix_session::Session;
use uuid::Uuid;

pub async fn from_session(session: Session) -> anyhow::Result<Uuid> {
    if let Some(uid) = session.get::<Uuid>("current_uid")? {
        return Ok(uid);
    }
    Err(anyhow::anyhow!("no uid in session"))
}

pub async fn to_session(session: Session, uid: Uuid) -> anyhow::Result<()> {
    session.insert("current_uid", uid)?;
    match session.get::<Vec<Uuid>>("users_uid") {
        Ok(Some(mut uids)) => {
            if !uids.contains(&uid) {
                uids.push(uid);
                session.insert("users_uid", uids)?;
            }
        }
        Ok(None) => {
            session.insert("users_uid", vec![uid])?;
        }
        Err(_) => {
            session.insert("users_uid", vec![uid])?;
        }
    }
    Ok(())
}
