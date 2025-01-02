use std::fmt::{Debug, Display};
use std::io;

pub type JZResult<T> = Result<T, JZError>;

pub enum JZError {
    IoErr(io::Error),
    GitErr(git2::Error),
    DbErr(sea_orm::DbErr),
    SqlErr(sea_orm::SqlErr),
    SqlxErr(sea_orm::SqlxError),
    ActixErr(actix_web::Error),
    Other(anyhow::Error),
}

impl From<io::Error> for JZError {
    fn from(err: io::Error) -> Self {
        JZError::IoErr(err)
    }
}
impl From<git2::Error> for JZError {
    fn from(err: git2::Error) -> Self {
        JZError::GitErr(err)
    }
}
impl From<sea_orm::DbErr> for JZError {
    fn from(err: sea_orm::DbErr) -> Self {
        JZError::DbErr(err)
    }
}

impl From<sea_orm::SqlErr> for JZError {
    fn from(err: sea_orm::SqlErr) -> Self {
        JZError::SqlErr(err)
    }
}

impl From<sea_orm::SqlxError> for JZError {
    fn from(err: sea_orm::SqlxError) -> Self {
        JZError::SqlxErr(err)
    }
}
impl From<anyhow::Error> for JZError {
    fn from(err: anyhow::Error) -> Self {
        JZError::Other(err)
    }
}
impl From<actix_web::Error> for JZError {
    fn from(err: actix_web::Error) -> Self {
        JZError::ActixErr(err)
    }
}
impl Into<actix_web::Error> for JZError {
    fn into(self) -> actix_web::Error {
        match self {
            JZError::IoErr(err) => actix_web::error::ErrorInternalServerError(err),
            JZError::GitErr(err) => actix_web::error::ErrorInternalServerError(err),
            JZError::DbErr(err) => actix_web::error::ErrorInternalServerError(err),
            JZError::SqlErr(err) => actix_web::error::ErrorInternalServerError(err),
            JZError::SqlxErr(err) => actix_web::error::ErrorInternalServerError(err),
            JZError::Other(err) => actix_web::error::ErrorInternalServerError(err),
            JZError::ActixErr(err) => err,
        }
    }
}
impl Debug for JZError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JZError::IoErr(err) => write!(f, "IoErr: {}", err),
            JZError::GitErr(err) => write!(f, "GitErr: {}", err),
            JZError::DbErr(err) => write!(f, "DbErr: {}", err),
            JZError::SqlErr(err) => write!(f, "SqlErr: {}", err),
            JZError::SqlxErr(err) => write!(f, "SqlxErr: {}", err),
            JZError::Other(err) => write!(f, "Other: {}", err),
            JZError::ActixErr(err) => write!(f, "ActixErr: {}", err),
        }
    }
}
impl Display for JZError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JZError::IoErr(err) => write!(f, "IoErr: {}", err),
            JZError::GitErr(err) => write!(f, "GitErr: {}", err),
            JZError::DbErr(err) => write!(f, "DbErr: {}", err),
            JZError::SqlErr(err) => write!(f, "SqlErr: {}", err),
            JZError::SqlxErr(err) => write!(f, "SqlxErr: {}", err),
            JZError::Other(err) => write!(f, "Other: {}", err),
            JZError::ActixErr(err) => write!(f, "ActixErr: {}", err),
        }
    }
}
