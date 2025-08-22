use actix_web::cookie::{Key, SameSite, time::Duration};
use derive_more::derive::From;

use crate::{SessionMiddleware, storage::SessionStore};
#[derive(Debug, Clone, From)]
#[non_exhaustive]
pub enum SessionLifecycle {
    BrowserSession(BrowserSession),
    PersistentSession(PersistentSession),
}

#[derive(Debug, Clone)]
pub struct BrowserSession {
    state_ttl: Duration,
    state_ttl_extension_policy: TtlExtensionPolicy,
}

impl BrowserSession {
    pub fn state_ttl(mut self, ttl: Duration) -> Self {
        self.state_ttl = ttl;
        self
    }
    pub fn state_ttl_extension_policy(mut self, ttl_extension_policy: TtlExtensionPolicy) -> Self {
        self.state_ttl_extension_policy = ttl_extension_policy;
        self
    }
}

impl Default for BrowserSession {
    fn default() -> Self {
        Self {
            state_ttl: default_ttl(),
            state_ttl_extension_policy: default_ttl_extension_policy(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PersistentSession {
    session_ttl: Duration,
    ttl_extension_policy: TtlExtensionPolicy,
}

impl PersistentSession {
    #[doc(alias = "max_age", alias = "max age", alias = "expires")]
    pub fn session_ttl(mut self, session_ttl: Duration) -> Self {
        self.session_ttl = session_ttl;
        self
    }
    pub fn session_ttl_extension_policy(
        mut self,
        ttl_extension_policy: TtlExtensionPolicy,
    ) -> Self {
        self.ttl_extension_policy = ttl_extension_policy;
        self
    }
}

impl Default for PersistentSession {
    fn default() -> Self {
        Self {
            session_ttl: default_ttl(),
            ttl_extension_policy: default_ttl_extension_policy(),
        }
    }
}
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TtlExtensionPolicy {
    OnEveryRequest,
    OnStateChanges,
}

#[derive(Debug, Clone, Copy)]
pub enum CookieContentSecurity {
    Private,
    Signed,
}

pub(crate) const fn default_ttl() -> Duration {
    Duration::days(1)
}

pub(crate) const fn default_ttl_extension_policy() -> TtlExtensionPolicy {
    TtlExtensionPolicy::OnStateChanges
}

#[must_use]
pub struct SessionMiddlewareBuilder<Store: SessionStore> {
    storage_backend: Store,
    configuration: Configuration,
}

impl<Store: SessionStore> SessionMiddlewareBuilder<Store> {
    pub(crate) fn new(store: Store, configuration: Configuration) -> Self {
        Self {
            storage_backend: store,
            configuration,
        }
    }

    pub fn cookie_name(mut self, name: String) -> Self {
        self.configuration.cookie.name = name;
        self
    }

    pub fn cookie_secure(mut self, secure: bool) -> Self {
        self.configuration.cookie.secure = secure;
        self
    }

    pub fn session_lifecycle<S: Into<SessionLifecycle>>(mut self, session_lifecycle: S) -> Self {
        match session_lifecycle.into() {
            SessionLifecycle::BrowserSession(BrowserSession {
                state_ttl,
                state_ttl_extension_policy,
            }) => {
                self.configuration.cookie.max_age = None;
                self.configuration.session.state_ttl = state_ttl;
                self.configuration.ttl_extension_policy = state_ttl_extension_policy;
            }
            SessionLifecycle::PersistentSession(PersistentSession {
                session_ttl,
                ttl_extension_policy,
            }) => {
                self.configuration.cookie.max_age = Some(session_ttl);
                self.configuration.session.state_ttl = session_ttl;
                self.configuration.ttl_extension_policy = ttl_extension_policy;
            }
        }

        self
    }

    pub fn cookie_same_site(mut self, same_site: SameSite) -> Self {
        self.configuration.cookie.same_site = same_site;
        self
    }

    pub fn cookie_path(mut self, path: String) -> Self {
        self.configuration.cookie.path = path;
        self
    }

    pub fn cookie_domain(mut self, domain: Option<String>) -> Self {
        self.configuration.cookie.domain = domain;
        self
    }

    pub fn cookie_content_security(mut self, content_security: CookieContentSecurity) -> Self {
        self.configuration.cookie.content_security = content_security;
        self
    }

    pub fn cookie_http_only(mut self, http_only: bool) -> Self {
        self.configuration.cookie.http_only = http_only;
        self
    }

    #[must_use]
    pub fn build(self) -> SessionMiddleware<Store> {
        SessionMiddleware::from_parts(self.storage_backend, self.configuration)
    }
}

#[derive(Clone)]
pub(crate) struct Configuration {
    pub(crate) cookie: CookieConfiguration,
    pub(crate) session: SessionConfiguration,
    pub(crate) ttl_extension_policy: TtlExtensionPolicy,
}

#[derive(Clone)]
pub(crate) struct SessionConfiguration {
    pub(crate) state_ttl: Duration,
}

#[derive(Clone)]
pub(crate) struct CookieConfiguration {
    pub(crate) secure: bool,
    pub(crate) http_only: bool,
    pub(crate) name: String,
    pub(crate) same_site: SameSite,
    pub(crate) path: String,
    pub(crate) domain: Option<String>,
    pub(crate) max_age: Option<Duration>,
    pub(crate) content_security: CookieContentSecurity,
    pub(crate) key: Key,
}

pub(crate) fn default_configuration(key: Key) -> Configuration {
    Configuration {
        cookie: CookieConfiguration {
            secure: true,
            http_only: true,
            name: "id".into(),
            same_site: SameSite::Lax,
            path: "/".into(),
            domain: None,
            max_age: None,
            content_security: CookieContentSecurity::Private,
            key,
        },
        session: SessionConfiguration {
            state_ttl: default_ttl(),
        },
        ttl_extension_policy: default_ttl_extension_policy(),
    }
}
