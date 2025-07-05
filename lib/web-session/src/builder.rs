use actix_utils::future::{Ready, ready};
use actix_web::dev::{Extensions, Payload, ServiceRequest, ServiceResponse};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use dashmap::DashMap;
use session::{Session, SessionInner, SessionStatus};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub struct WebSession(pub Session);

impl WebSession {
    pub fn set_session(
        req: &mut ServiceRequest,
        data: impl IntoIterator<Item = (String, String)>,
    ) -> anyhow::Result<()> {
        if let Some(s_impl) = req.extensions().get::<Rc<RefCell<SessionInner>>>() {
            let mut inner = s_impl.borrow_mut();
            inner.map.extend(data);
        } else {
            let inner = Rc::new(RefCell::new(SessionInner::default()));
            inner.borrow_mut().map.extend(data);
            req.extensions_mut().insert(inner);
        }
        Ok(())
    }
    pub fn get_changes<B>(
        res: &mut ServiceResponse<B>,
    ) -> (SessionStatus, DashMap<String, String>) {
        if let Some(s_impl) = res
            .request()
            .extensions()
            .get::<Rc<RefCell<SessionInner>>>()
        {
            let state = mem::take(&mut s_impl.borrow_mut().map);
            (s_impl.borrow().status.clone(), state)
        } else {
            (SessionStatus::Unchanged, DashMap::new())
        }
    }
    pub fn get_session(extensions: &mut Extensions) -> WebSession {
        if let Some(s_impl) = extensions.get::<Rc<RefCell<SessionInner>>>() {
            return WebSession(Session(Rc::clone(s_impl)));
        }
        let inner = Rc::new(RefCell::new(SessionInner::default()));
        extensions.insert(inner.clone());
        WebSession(Session(Rc::clone(&inner)))
    }
}

impl FromRequest for WebSession {
    type Error = Error;
    type Future = Ready<Result<WebSession, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(WebSession::get_session(&mut req.extensions_mut())))
    }
}
