use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha256::Sha256Digest;
use jz_module::AppModule;
use crate::utils::request::RequestBody;
use crate::utils::session::from_session;

#[derive(Deserialize,Serialize)]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
}


pub async fn update_password(
    ctx: Session,
    update_password: RequestBody<UpdatePassword>,
    app: Data<AppModule>
)
-> impl Responder {
    let opsuid = from_session(ctx).await;
    if opsuid.is_err() {
        return HttpResponse::Ok().json(json!({
            "code": 401,
            "message": "未登录",
        }))
    };
    let opsuid = opsuid.unwrap();
    let inner = update_password.into_inner().inner;
    if inner.old_password.is_empty() || inner.new_password.is_empty() {
        return HttpResponse::Ok().json(json!({
            "code": 400,
            "message": "密码不能为空",
        }))
    }
    if inner.old_password == inner.new_password {
        return HttpResponse::Ok().json(json!({
            "code": 400,
            "message": "新密码不能与旧密码相同",
        }))
    }
    let user = app.user_info_by_id(opsuid).await;
    if let Err(e) = user {
        return HttpResponse::Ok().json(json!({
            "code": 500,
            "message": e.to_string(),
        }))
    }
    let user = user.unwrap();
    if user.password != inner.old_password.digest() {
        return HttpResponse::Ok().json(json!({
            "code": 400,
            "message": "旧密码错误",
        }))
    }
    if let Err(e) = app.profile_update_password(opsuid, inner.new_password.digest()).await {
        HttpResponse::Ok().json(json!({
            "code": 500,
            "message": e.to_string(),
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "修改成功",
        }))
    }
}
