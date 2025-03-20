use actix_web::{web, HttpResponse};
use serde_json::json;
use jz_module::AppModule;

pub async fn check_name(
    app: web::Data<AppModule>,
    path: web::Path<String>,
)
-> impl actix_web::Responder
{
    match app.check_name(path.to_string()).await {
        Ok(x) => {
            HttpResponse::Ok().json(json!({
                "code": 0,
                "data": x,
            }))
        }
        Err(_) => {
            HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": "error",
            }))
        }
    }
}