use actix_web::web;
use crate::metadata::service::MetaService;

pub mod init;
pub mod handler;
pub mod dto;
pub mod middleware;
pub mod app_write;
pub mod app_error;
pub mod app_routes;
pub mod app_docs;
pub mod graphql;

pub type SERVER = web::Data<MetaService>;