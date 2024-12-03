use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use serde_json::{Number, Value};

pub struct R<T: Serialize>{
    pub code: u16,
    pub msg: Option<String>,
    pub data: Option<T>,
}


impl <T>Responder for R<T>
where T: Serialize
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut value = Value::default();
        value["code"] = Value::Number(Number::from(self.code));
        if let Some(msg) = self.msg {
            value["msg"] = Value::String(msg);
        }
        if let Some(data) = self.data {
            value["data"] = Value::Object(serde_json::to_value(data).unwrap().as_object().unwrap().clone());
        }
        HttpResponse::new(StatusCode::OK)
            .set_body(BoxBody::new(value.to_string()))
    }
}

#[allow(non_snake_case)]
pub fn Ok() -> R<String> {
    R::<String>{
        code: 200,
        msg: Some(String::from("[Ok]")),
        data: None,
    }
}

#[allow(non_snake_case)]
pub fn OkMsg(msg: String) -> R<String> {
    R::<String>{
        code: 200,
        msg: Some(String::from(msg)),
        data: None,
    }
}
#[allow(non_snake_case)]
pub fn BadRequest(code: u16, msg: Option<String>) -> R<String> {
    R::<String>{
        code,
        msg,
        data: None,
    }
}

#[allow(non_snake_case)]
pub fn Err() -> R<String> {
    R::<String>{
        code: 403,
        msg: None,
        data: None,
    }
}
#[allow(non_snake_case)]
pub fn ErrMsg(msg: String) -> R<String> {
    R::<String>{
        code: 403,
        msg: Some(String::from(msg)),
        data: None,
    }
}
#[allow(non_snake_case)]
pub fn NotFound() -> R<String> {
    R::<String>{
        code: 404,
        msg: None,
        data: None,
    }
}