use actix_session::SessionExt;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::middleware::Next;
use crate::api::middleware::session::{SessionModel, SESSION_USER_KEY};

pub async fn must_login(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let session = req.get_session();
    if session.get::<SessionModel>(SESSION_USER_KEY)?.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    next.call(req).await
}