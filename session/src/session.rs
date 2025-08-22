use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    error::Error as StdError,
    mem,
    rc::Rc,
};

use actix_utils::future::{Ready, ready};
use actix_web::{
    FromRequest, HttpMessage, HttpRequest, HttpResponse, ResponseError,
    body::BoxBody,
    dev::{Extensions, Payload, ServiceRequest, ServiceResponse},
    error::Error,
};
use anyhow::Context;
use derive_more::derive::{Display, From};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Clone)]
pub struct Session(Rc<RefCell<SessionInner>>);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum SessionStatus {
    Changed,
    Purged,
    Renewed,
    #[default]
    Unchanged,
}

#[derive(Default)]
struct SessionInner {
    state: HashMap<String, String>,
    status: SessionStatus,
}

impl Session {
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, SessionGetError> {
        if let Some(val_str) = self.0.borrow().state.get(key) {
            Ok(Some(
                serde_json::from_str(val_str)
                    .with_context(|| {
                        format!(
                            "Failed to deserialize the JSON-encoded session data attached to key \
                            `{}` as a `{}` type",
                            key,
                            std::any::type_name::<T>()
                        )
                    })
                    .map_err(SessionGetError)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.borrow().state.contains_key(key)
    }

    pub fn entries(&self) -> Ref<'_, HashMap<String, String>> {
        Ref::map(self.0.borrow(), |inner| &inner.state)
    }

    pub fn status(&self) -> SessionStatus {
        Ref::map(self.0.borrow(), |inner| &inner.status).clone()
    }

    pub fn insert<T: Serialize>(
        &self,
        key: impl Into<String>,
        value: T,
    ) -> Result<(), SessionInsertError> {
        let mut inner = self.0.borrow_mut();

        if inner.status != SessionStatus::Purged {
            if inner.status != SessionStatus::Renewed {
                inner.status = SessionStatus::Changed;
            }

            let key = key.into();
            let val = serde_json::to_string(&value)
                .with_context(|| {
                    format!(
                        "Failed to serialize the provided `{}` type instance as JSON in order to \
                        attach as session data to the `{key}` key",
                        std::any::type_name::<T>(),
                    )
                })
                .map_err(SessionInsertError)?;

            inner.state.insert(key, val);
        }

        Ok(())
    }

    pub fn update<T: Serialize + DeserializeOwned, F>(
        &self,
        key: impl Into<String>,
        updater: F,
    ) -> Result<(), SessionUpdateError>
    where
        F: FnOnce(T) -> T,
    {
        let mut inner = self.0.borrow_mut();
        let key_str = key.into();

        if let Some(val_str) = inner.state.get(&key_str) {
            let value = serde_json::from_str(val_str)
                .with_context(|| {
                    format!(
                        "Failed to deserialize the JSON-encoded session data attached to key \
                        `{key_str}` as a `{}` type",
                        std::any::type_name::<T>()
                    )
                })
                .map_err(SessionUpdateError)?;

            let val = serde_json::to_string(&updater(value))
                .with_context(|| {
                    format!(
                        "Failed to serialize the provided `{}` type instance as JSON in order to \
                        attach as session data to the `{key_str}` key",
                        std::any::type_name::<T>(),
                    )
                })
                .map_err(SessionUpdateError)?;

            inner.state.insert(key_str, val);
        }

        Ok(())
    }

    pub fn update_or<T: Serialize + DeserializeOwned, F>(
        &self,
        key: &str,
        default_value: T,
        updater: F,
    ) -> Result<(), SessionUpdateError>
    where
        F: FnOnce(T) -> T,
    {
        if self.contains_key(key) {
            self.update(key, updater)
        } else {
            self.insert(key, default_value)
                .map_err(|err| SessionUpdateError(err.into()))
        }
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        let mut inner = self.0.borrow_mut();

        if inner.status != SessionStatus::Purged {
            if inner.status != SessionStatus::Renewed {
                inner.status = SessionStatus::Changed;
            }
            return inner.state.remove(key);
        }

        None
    }
    pub fn remove_as<T: DeserializeOwned>(&self, key: &str) -> Option<Result<T, String>> {
        self.remove(key)
            .map(|val_str| match serde_json::from_str(&val_str) {
                Ok(val) => Ok(val),
                Err(_err) => {
                    tracing::debug!(
                        "Removed value (key: {}) could not be deserialized as {}",
                        key,
                        std::any::type_name::<T>()
                    );

                    Err(val_str)
                }
            })
    }

    pub fn clear(&self) {
        let mut inner = self.0.borrow_mut();

        if inner.status != SessionStatus::Purged {
            if inner.status != SessionStatus::Renewed {
                inner.status = SessionStatus::Changed;
            }
            inner.state.clear()
        }
    }

    pub fn purge(&self) {
        let mut inner = self.0.borrow_mut();
        inner.status = SessionStatus::Purged;
        inner.state.clear();
    }

    pub fn renew(&self) {
        let mut inner = self.0.borrow_mut();

        if inner.status != SessionStatus::Purged {
            inner.status = SessionStatus::Renewed;
        }
    }

    #[allow(clippy::needless_pass_by_ref_mut)]
    pub(crate) fn set_session(
        req: &mut ServiceRequest,
        data: impl IntoIterator<Item = (String, String)>,
    ) {
        let session = Session::get_session(&mut req.extensions_mut());
        let mut inner = session.0.borrow_mut();
        inner.state.extend(data);
    }

    #[allow(clippy::needless_pass_by_ref_mut)]
    pub(crate) fn get_changes<B>(
        res: &mut ServiceResponse<B>,
    ) -> (SessionStatus, HashMap<String, String>) {
        if let Some(s_impl) = res
            .request()
            .extensions()
            .get::<Rc<RefCell<SessionInner>>>()
        {
            let state = mem::take(&mut s_impl.borrow_mut().state);
            (s_impl.borrow().status.clone(), state)
        } else {
            (SessionStatus::Unchanged, HashMap::new())
        }
    }

    pub(crate) fn get_session(extensions: &mut Extensions) -> Session {
        if let Some(s_impl) = extensions.get::<Rc<RefCell<SessionInner>>>() {
            return Session(Rc::clone(s_impl));
        }

        let inner = Rc::new(RefCell::new(SessionInner::default()));
        extensions.insert(inner.clone());

        Session(inner)
    }
}

impl FromRequest for Session {
    type Error = Error;
    type Future = Ready<Result<Session, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(Session::get_session(&mut req.extensions_mut())))
    }
}

#[derive(Debug, Display, From)]
#[display("{_0}")]
pub struct SessionGetError(anyhow::Error);

impl StdError for SessionGetError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.0.as_ref())
    }
}

impl ResponseError for SessionGetError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::new(self.status_code())
    }
}

#[derive(Debug, Display, From)]
#[display("{_0}")]
pub struct SessionInsertError(anyhow::Error);

impl StdError for SessionInsertError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.0.as_ref())
    }
}

impl ResponseError for SessionInsertError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::new(self.status_code())
    }
}

#[derive(Debug, Display, From)]
#[display("{_0}")]
pub struct SessionUpdateError(anyhow::Error);

impl StdError for SessionUpdateError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.0.as_ref())
    }
}

impl ResponseError for SessionUpdateError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::new(self.status_code())
    }
}
