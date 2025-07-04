use std::fmt;
use std::pin::Pin;
use std::rc::Rc;
use actix_web::body::MessageBody;
use actix_web::cookie::{Cookie, CookieJar, Key};
use actix_web::dev::{forward_ready, ResponseHead, Service, ServiceRequest, ServiceResponse, Transform};
use session::storage::SessionStorage;
use crate::config;
use crate::config::{Configuration, CookieConfiguration, CookieContentSecurity, SessionMiddlewareBuilder, TtlExtensionPolicy};
use actix_utils::future::{ready, Ready};
use actix_web::http::header::{HeaderValue, SET_COOKIE};
use actix_web::HttpResponse;
use anyhow::Context;
use dashmap::DashMap;
use session::SessionStatus;
use crate::builder::WebSession;

#[derive(Clone)]
pub struct SessionMiddleware<Store: SessionStorage> {
    storage_backend: Rc<Store>,
    configuration: Rc<Configuration>,
}

impl<Store: SessionStorage> SessionMiddleware<Store> {
    /// Use [`SessionMiddleware::new`] to initialize the session framework using the default
    /// parameters.
    ///
    /// To create a new instance of [`SessionMiddleware`] you need to provide:
    /// - an instance of the session storage backend you wish to use (i.e. an implementation of
    ///   [`SessionStore`]);
    /// - a secret key, to sign or encrypt the content of client-side session cookie.
    pub fn new(store: Store, key: Key) -> Self {
        Self::builder(store, key).build()
    }

    /// A fluent API to configure [`SessionMiddleware`].
    ///
    /// It takes as input the two required inputs to create a new instance of [`SessionMiddleware`]:
    /// - an instance of the session storage backend you wish to use (i.e. an implementation of
    ///   [`SessionStore`]);
    /// - a secret key, to sign or encrypt the content of client-side session cookie.
    pub fn builder(store: Store, key: Key) -> SessionMiddlewareBuilder<Store> {
        SessionMiddlewareBuilder::new(store, config::default_configuration(key))
    }

    pub(crate) fn from_parts(store: Store, configuration: Configuration) -> Self {
        Self {
            storage_backend: Rc::new(store),
            configuration: Rc::new(configuration),
        }
    }
}

impl<S, B, Store> Transform<S, ServiceRequest> for SessionMiddleware<Store>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    Store: SessionStorage + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = InnerSessionMiddleware<S, Store>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InnerSessionMiddleware {
            service: Rc::new(service),
            configuration: Rc::clone(&self.configuration),
            storage_backend: Rc::clone(&self.storage_backend),
        }))
    }
}

fn e500<E: fmt::Debug + fmt::Display + 'static>(err: E) -> actix_web::Error {
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::InternalServerError().finish(),
    )
        .into()
}

#[doc(hidden)]
#[non_exhaustive]
pub struct InnerSessionMiddleware<S, Store: SessionStorage + 'static> {
    service: Rc<S>,
    configuration: Rc<Configuration>,
    storage_backend: Rc<Store>,
}

impl<S, B, Store> Service<ServiceRequest> for InnerSessionMiddleware<S, Store>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    Store: SessionStorage + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let storage_backend = Rc::clone(&self.storage_backend);
        let configuration = Rc::clone(&self.configuration);

        Box::pin(async move {
            let session_key = extract_session_key(&req, &configuration.cookie);
            let (session_key, session_state) =
                load_session_state(session_key, storage_backend.as_ref()).await?;

            WebSession::set_session(&mut req, session_state).ok();

            let mut res = service.call(req).await?;
            let (status, session_state) = WebSession::get_changes(&mut res);

            match session_key {
                None => {
                    if !session_state.is_empty() {
                        let session_key = dbg!(
                            storage_backend
                                .save(&session_state, &configuration.session.state_ttl)
                                .await
                        )
                            .map_err(e500)?;

                        set_session_cookie(
                            res.response_mut().head_mut(),
                            session_key,
                            &configuration.cookie,
                        )
                            .map_err(e500)?;
                    }
                }

                Some(session_key) => {
                    match status {
                        SessionStatus::Change => {
                            let session_key = storage_backend
                                .update(
                                    &session_key,
                                    &session_state,
                                    &configuration.session.state_ttl,
                                )
                                .await
                                .map_err(e500)?;

                            set_session_cookie(
                                res.response_mut().head_mut(),
                                session_key,
                                &configuration.cookie,
                            )
                                .map_err(e500)?;
                        }

                        SessionStatus::Purge => {
                            storage_backend.delete(&session_key).await.map_err(e500)?;

                            delete_session_cookie(
                                res.response_mut().head_mut(),
                                &configuration.cookie,
                            )
                                .map_err(e500)?;
                        }

                        SessionStatus::Renewed => {
                            storage_backend.delete(&session_key).await.map_err(e500)?;

                            let session_key = storage_backend
                                .save(&session_state, &configuration.session.state_ttl)
                                .await
                                .map_err(e500)?;

                            set_session_cookie(
                                res.response_mut().head_mut(),
                                session_key,
                                &configuration.cookie,
                            )
                                .map_err(e500)?;
                        }

                        SessionStatus::Unchanged => {
                            if matches!(
                                configuration.ttl_extension_policy,
                                TtlExtensionPolicy::OnEveryRequest
                            ) {
                                storage_backend
                                    .update_ttl(&session_key, configuration.session.state_ttl)
                                    .await
                                    .map_err(e500)?;

                                if configuration.cookie.max_age.is_some() {
                                    set_session_cookie(
                                        res.response_mut().head_mut(),
                                        session_key,
                                        &configuration.cookie,
                                    )
                                        .map_err(e500)?;
                                }
                            }
                        }
                    };
                }
            }

            Ok(res)
        })
    }
}
fn extract_session_key(req: &ServiceRequest, config: &CookieConfiguration) -> Option<String> {
    let cookies = req.cookies().ok()?;
    let session_cookie = cookies
        .iter()
        .find(|&cookie| cookie.name() == config.name)?;

    let mut jar = CookieJar::new();
    jar.add_original(session_cookie.clone());

    let verification_result = match config.content_security {
        CookieContentSecurity::Signed => jar.signed(&config.key).get(&config.name),
        CookieContentSecurity::Private => jar.private(&config.key).get(&config.name),
    };

    if verification_result.is_none() {
        tracing::warn!(
            "The session cookie attached to the incoming request failed to pass cryptographic \
            checks (signature verification/decryption)."
        );
    }

    match verification_result?.value().to_owned().try_into() {
        Ok(session_key) => Some(session_key),
        Err(err) => {
            tracing::warn!(
                error.message = %err,
                error.cause_chain = ?err,
                "Invalid session key, ignoring."
            );

            None
        }
    }
}

async fn load_session_state<Store: SessionStorage>(
    session_key: Option<String>,
    storage_backend: &Store,
) -> Result<(Option<String>, DashMap<String, String>), actix_web::Error> {
    if let Some(session_key) = session_key {
        match storage_backend.load(&session_key).await {
            Ok(state) => {
                Ok((Some(session_key), state))
            }

            Err(_) => {
                Ok((Some(session_key), DashMap::new()))
            },
        }
    } else {
        Ok((None, DashMap::new()))
    }
}

fn set_session_cookie(
    response: &mut ResponseHead,
    session_key: String,
    config: &CookieConfiguration,
) -> Result<(), anyhow::Error> {
    let value: String = session_key.into();
    let mut cookie = Cookie::new(config.name.clone(), value);

    cookie.set_secure(config.secure);
    cookie.set_http_only(config.http_only);
    cookie.set_same_site(config.same_site);
    cookie.set_path(config.path.clone());

    if let Some(max_age) = config.max_age {
        cookie.set_max_age(max_age);
    }

    if let Some(ref domain) = config.domain {
        cookie.set_domain(domain.clone());
    }

    let mut jar = CookieJar::new();
    match config.content_security {
        CookieContentSecurity::Signed => jar.signed_mut(&config.key).add(cookie),
        CookieContentSecurity::Private => jar.private_mut(&config.key).add(cookie),
    }

    // set cookie
    let cookie = jar.delta().next().unwrap();
    let val = HeaderValue::from_str(&cookie.encoded().to_string())
        .context("Failed to attach a session cookie to the outgoing response")?;

    response.headers_mut().append(SET_COOKIE, val);

    Ok(())
}

fn delete_session_cookie(
    response: &mut ResponseHead,
    config: &CookieConfiguration,
) -> Result<(), anyhow::Error> {
    let removal_cookie = Cookie::build(config.name.clone(), "")
        .path(config.path.clone())
        .secure(config.secure)
        .http_only(config.http_only)
        .same_site(config.same_site);

    let mut removal_cookie = if let Some(ref domain) = config.domain {
        removal_cookie.domain(domain)
    } else {
        removal_cookie
    }
        .finish();

    removal_cookie.make_removal();

    let val = HeaderValue::from_str(&removal_cookie.to_string())
        .context("Failed to attach a session removal cookie to the outgoing response")?;
    response.headers_mut().append(SET_COOKIE, val);

    Ok(())
}