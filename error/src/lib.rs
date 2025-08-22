use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AppError {
    pub code: i32,
    pub msg: String,
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError {
            code: 901,
            msg: err.to_string(),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError {
            code: 902,
            msg: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError {
            code: 903,
            msg: err.to_string(),
        }
    }
}

impl From<sea_orm::SqlErr> for AppError {
    fn from(err: sea_orm::SqlErr) -> Self {
        AppError {
            code: 904,
            msg: err.to_string(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError {
            code: 905,
            msg: err.to_string(),
        }
    }
}

impl From<git2::Error> for AppError {
    fn from(err: git2::Error) -> Self {
        AppError {
            code: 906,
            msg: err.to_string(),
        }
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError {
            code: 907,
            msg: err.to_string(),
        }
    }
}

pub trait AppResult {
    fn into_response(self) -> HttpResponse;
}

impl<T> AppResult for Result<T, AppError>
where
    T: Serialize,
{
    fn into_response(self) -> HttpResponse {
        match self {
            Ok(data) => HttpResponse::Ok().json(json!({
                "code": 200,
                "data": data,
                "msg": "success"
            })),
            Err(err) => HttpResponse::Ok().json(json!({
                "code": err.code,
                "msg": err.msg,
                "data": {}
            })),
        }
    }
}
