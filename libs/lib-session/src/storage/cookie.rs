use actix_web::cookie::time::Duration;
use anyhow::Error;

use super::SessionKey;
use crate::storage::interface::LoadError;
use crate::storage::interface::SaveError;
use crate::storage::interface::SessionState;
use crate::storage::interface::UpdateError;
use crate::storage::SessionStore;

#[derive(Default)]
#[non_exhaustive]
pub struct CookieSessionStore;

impl SessionStore for CookieSessionStore {
    async fn load(&self, session_key : &SessionKey) -> Result<Option<SessionState>, LoadError> {
        serde_json::from_str(session_key.as_ref())
            .map(Some)
            .map_err(anyhow::Error::new)
            .map_err(LoadError::Deserialization)
    }

    async fn save(
        &self,
        session_state : SessionState,
        _ttl : &Duration,
    ) -> Result<SessionKey, SaveError> {
        let session_key = serde_json::to_string(&session_state)
            .map_err(anyhow::Error::new)
            .map_err(SaveError::Serialization)?;

        session_key
            .try_into()
            .map_err(Into::into)
            .map_err(SaveError::Other)
    }

    async fn update(
        &self,
        _session_key : SessionKey,
        session_state : SessionState,
        ttl : &Duration,
    ) -> Result<SessionKey, UpdateError> {
        self.save(session_state, ttl)
            .await
            .map_err(|err| match err {
                SaveError::Serialization(err) => UpdateError::Serialization(err),
                SaveError::Other(err) => UpdateError::Other(err),
            })
    }

    async fn update_ttl(&self, _session_key : &SessionKey, _ttl : &Duration) -> Result<(), Error> {
        Ok(())
    }

    async fn delete(&self, _session_key : &SessionKey) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
