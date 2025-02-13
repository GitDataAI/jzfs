use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::body::BoxBody;
use serde::Serialize;
use serde_json::Number;
use serde_json::Value;
use serde_json::json;

#[derive(Serialize)]
pub struct AppWrite<T : Serialize> {
    pub code : u16,
    pub msg : String,
    pub data : Option<T>,
}

impl<T> AppWrite<T>
where
    T : Serialize,
{
    pub fn new(code : u16, msg : String, data : Option<T>) -> Self {
        AppWrite { code, msg, data }
    }
    pub fn ok(data : T) -> Self {
        AppWrite::new(200, "ok".to_string(), Some(data))
    }
    pub fn ok_msg(msg : String) -> Self {
        AppWrite::new(200, msg, None)
    }
    pub fn ok_data(data : T) -> Self {
        AppWrite::new(200, "ok".to_string(), Some(data))
    }
    pub fn fail(msg : String) -> Self {
        AppWrite::new(500, msg, None)
    }
    pub fn error(msg : String) -> Self {
        AppWrite::new(500, msg, None)
    }
    pub fn success(msg : String) -> Self {
        AppWrite::new(200, msg, None)
    }
    pub fn not_found(msg : String) -> Self {
        AppWrite::new(404, msg, None)
    }
    pub fn unauthorized(msg : String) -> Self {
        AppWrite::new(401, msg, None)
    }
    pub fn bad_request(msg : String) -> Self {
        AppWrite::new(400, msg, None)
    }
    pub fn internal_server_error(msg : String) -> Self {
        AppWrite::new(500, msg, None)
    }
    pub fn service_unavailable(msg : String) -> Self {
        AppWrite::new(503, msg, None)
    }
    pub fn gateway_timeout(msg : String) -> Self {
        AppWrite::new(504, msg, None)
    }
    pub fn forbidden(msg : String) -> Self {
        AppWrite::new(403, msg, None)
    }
    pub fn unprocessable_entity(msg : String) -> Self {
        AppWrite::new(422, msg, None)
    }
    pub fn conflict(msg : String) -> Self {
        AppWrite::new(409, msg, None)
    }
    pub fn not_implemented(msg : String) -> Self {
        AppWrite::new(501, msg, None)
    }
}

impl<T> From<T> for AppWrite<T>
where
    T : Serialize,
{
    fn from(data : T) -> Self {
        AppWrite::ok(data)
    }
}

impl<T> From<Result<T, String>> for AppWrite<T>
where
    T : Serialize,
{
    fn from(result : Result<T, String>) -> Self {
        match result {
            Ok(data) => AppWrite::ok(data),
            Err(msg) => AppWrite::fail(msg),
        }
    }
}

impl<T> From<Result<T, Box<dyn std::error::Error>>> for AppWrite<T>
where
    T : Serialize,
{
    fn from(result : Result<T, Box<dyn std::error::Error>>) -> Self {
        match result {
            Ok(data) => AppWrite::ok(data),
            Err(msg) => AppWrite::fail(msg.to_string()),
        }
    }
}

impl<T> From<Result<T, std::io::Error>> for AppWrite<T>
where
    T : Serialize,
{
    fn from(result : Result<T, std::io::Error>) -> Self {
        match result {
            Ok(data) => AppWrite::ok(data),
            Err(msg) => AppWrite::fail(msg.to_string()),
        }
    }
}

// impl<T : serde::ser::Serialize> actix_web::Responder for AppWrite<T> {
//     type Body = BoxBody;
//
//     fn respond_to(self, _req : &HttpRequest) -> HttpResponse<Self::Body> {
//         let mut value = Value::default();
//         value["code"] = Value::Number(Number::from(self.code));
//         value["msg"] = Value::String(self.msg);
//         if let Some(data) = self.data {
//             value["data"] = json!(data);
//         }
//         HttpResponse::Ok()
//             .content_type("application/json")
//             .body(BoxBody::new(value.to_string()))
//     }
// }

impl<T : Serialize> Responder for AppWrite<T> {
    type Body = BoxBody;

    fn respond_to(self, req : &HttpRequest) -> HttpResponse<Self::Body> {
        let mut value = Value::default();
        value["code"] = Value::Number(Number::from(self.code));
        value["msg"] = Value::String(self.msg);
        if let Some(data) = self.data {
            value["data"] = json!(data);
        }
        HttpResponse::Ok()
            .insert_header(("Content-Type", "application/json"))
            .body(value.to_string())
            .respond_to(req)
    }
}
