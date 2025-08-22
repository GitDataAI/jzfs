use crate::AppCore;
use error::AppError;
use session::{Session, USER_KET, UserSession};

impl AppCore {
    pub async fn auth_user_logout(&self, session: Session) -> Result<(), AppError> {
        if let Ok(Some(_user_session)) = session.get::<UserSession>(USER_KET) {
            session.remove(USER_KET);
        } else {
            return Err(AppError::from(anyhow::anyhow!("user not login")));
        }
        Ok(())
    }
}
