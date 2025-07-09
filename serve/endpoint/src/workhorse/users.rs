use crate::endpoint::Endpoint;
use actix_web::HttpResponse;
use serde_json::json;
use workhorse::schema::users::UserCheckParam;

impl Endpoint {
    pub async fn user_check(&self, param: UserCheckParam) -> HttpResponse {
        let res = self.workhorse.user_check(self.new_context(), param).await;
        match res {
            Ok(res) => HttpResponse::Ok().json(json!({ "code": res.code })),
            Err(err) => HttpResponse::Ok().json(json!({ "code": 501, "msg": err.to_string() })),
        }
    }
}