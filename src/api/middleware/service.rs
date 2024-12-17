#![allow(unused_imports)]

use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use tracing::info;

pub struct ActixServer;

impl<S, B> Transform<S, ServiceRequest> for ActixServer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ActixServerHiMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ActixServerHiMiddleware { service }))
    }
}

pub struct ActixServerHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ActixServerHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("{}", format!("Path: {}, Method: {}, Cookies: {:?}", req.path(), req.method().to_string(),req.version()));
        
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut()
                .insert(
                    "Master".parse().unwrap(),
                    "GitDataAi".parse().unwrap()
                );
            Ok(res)
        })
    }
}