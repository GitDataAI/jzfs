use std::io;
use std::pin::Pin;
use std::task::Poll;

use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;

use crate::service::AppFsState;
use crate::transport::Transport;

pub mod pack;
pub mod refs;
pub mod text;

#[allow(dead_code)]
#[derive(Clone)]
pub struct HttpGit {
    pub(crate) service : AppFsState,
    pub(crate) transport : Transport,
    pub(crate) port : u16,
}

impl Future for HttpGit {
    type Output = io::Result<()>;
    fn poll(self: Pin<&mut Self>, cx : &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut future = Box::pin(self.get_mut().run());
        future.as_mut().poll(cx)
    }
}

impl HttpGit {
    pub fn new(service : AppFsState, transport : Transport, port : u16) -> Self {
        Self {
            service,
            transport,
            port,
        }
    }
    pub async fn run(&self) -> io::Result<()> {
        HttpServer::new(move || {
            App::new().service(
                web::scope("/{owner}/{repo}.git")
                    .route("/git-{service}-pack", web::to(pack::http_pack))
                    .route("/info/refs", web::to(refs::http_refs))
                    .route("/{path}", web::to(text::http_text)),
            )
        })
        .bind(format!("0.0.0.0:{}", self.port))?
        .run()
        .await?;
        Ok(())
    }
}
