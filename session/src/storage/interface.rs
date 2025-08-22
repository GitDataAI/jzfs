use std::{collections::HashMap, future::Future};

use actix_web::cookie::time::Duration;
use derive_more::derive::Display;

use super::SessionKey;

pub(crate) type SessionState = HashMap<String, String>;

pub trait SessionStore {
    fn load(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<Option<SessionState>, LoadError>>;

    fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> impl Future<Output = Result<SessionKey, SaveError>>;

    fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> impl Future<Output = Result<SessionKey, UpdateError>>;

    fn update_ttl(
        &self,
        session_key: &SessionKey,
        ttl: &Duration,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;

    fn delete(&self, session_key: &SessionKey) -> impl Future<Output = Result<(), anyhow::Error>>;
}

#[derive(Debug, Display)]
pub enum LoadError {
    #[display("Failed to deserialize session state")]
    Deserialization(anyhow::Error),

    #[display("Something went wrong when retrieving the session state")]
    Other(anyhow::Error),
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Deserialization(err) => Some(err.as_ref()),
            Self::Other(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug, Display)]
pub enum SaveError {
    #[display("Failed to serialize session state")]
    Serialization(anyhow::Error),

    #[display("Something went wrong when persisting the session state")]
    Other(anyhow::Error),
}

impl std::error::Error for SaveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(err) => Some(err.as_ref()),
            Self::Other(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug, Display)]
pub enum UpdateError {
    #[display("Failed to serialize session state")]
    Serialization(anyhow::Error),

    #[display("Something went wrong when updating the session state.")]
    Other(anyhow::Error),
}

impl std::error::Error for UpdateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(err) => Some(err.as_ref()),
            Self::Other(err) => Some(err.as_ref()),
        }
    }
}
