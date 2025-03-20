use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use serde_json::{json, Value};
use jz_module::AppModule;

pub async fn merge_orgs(
    path: Path<String>,
    app: Data<AppModule>
)
-> impl actix_web::Responder {
    let mut value = Value::Null;
    value["code"] = Value::from(0);
    if let Ok(org) = app.org_by_name(path.to_string()).await {
        value["data"]["orgs"] = json!(org);
    }else { 
        value["code"] = Value::from(1);
        value["msg"] = Value::from("Org not found");
        return HttpResponse::Ok().json(value);
    }
    if let Ok(members) = app.member_list_by_name(path.to_string()).await {
        value["data"]["members"] = json!(members);
    }else {
        value["code"] = Value::from(1);
        value["msg"] = Value::from("Org not found");
        return HttpResponse::Ok().json(value);
    }
    
    HttpResponse::Ok()
        .json(value)
}