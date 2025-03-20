use crate::utils::session::from_session;
use actix_session::Session;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, Responder};
use jz_module::AppModule;
use serde_json::{Value, json};

pub async fn merge_users(
    session: Session,
    module: Data<AppModule>,
    path: Path<String>,
) -> impl Responder {
    let mut value = Value::Null;
    value["code"] = Value::from(0);
    let userinfo = module.user_info_by_username(path.to_string()).await;
    if userinfo.is_err() {
        value["code"] = Value::from(1);
        value["msg"] = Value::from("用户不存在");
        return HttpResponse::Ok().json(value);
    }
    let info = userinfo.unwrap();
    if let Ok(owner) = from_session(session).await {
        if owner == info.uid {
            value["owner"] = Value::Bool(true);
            value["data"]["users"]["email"] = Value::from(info.email);
        } else {
            value["owner"] = Value::Bool(false);
            if info.show_email {
                value["data"]["users"]["email"] = Value::from(info.email);
            }
        }
    }
    let orgs = module.member_orgs(info.uid).await;
    if orgs.is_ok() {
        value["data"]["orgs"] = Value::from(json!(orgs.unwrap()));
    }
    value["data"]["users"] = json!({
        "uid": info.uid,
        "username": info.username,
        "avatar": info.avatar,
    });

    HttpResponse::Ok().json(value)
}
