use actix_session::SessionExt;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::Error;
use crate::api::middleware::session::ALLOW_NEXT_KEY;

pub async fn must_allow_next(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let session = req.get_session();
    let allow =  session.get::<bool>(ALLOW_NEXT_KEY)?;
    if allow.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    if !allow.unwrap() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    next.call(req).await
}