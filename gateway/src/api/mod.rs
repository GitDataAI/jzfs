use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Bytes, Payload};
use actix_session::Session;
use lib_config::naming::HttpServiceNode;




/*
 * Gateway 入口
 * @Author: ZhenYi
 */
pub async fn endpoint(request: HttpRequest, data: web::Data<Vec<HttpServiceNode>>, payload: Payload, session: Session) -> HttpResponse {
    let url = request.uri();
    let url = url.to_string();
    
    if let Ok(Some(x)) = session.get::<String>("index"){
        println!("{}", x);
    }
    let method = request.method();
    let header = request.headers();
    if url.starts_with("/api/"){
        let url = url.split("/").collect::<Vec<&str>>();
        if url.len() < 3 {
            return HttpResponse::NotFound().body("404 notfound");
        }
        let service = url[3];
        let path = url[3..].join("/");

        if let Some(server) = data.iter().find(|x| x.endpoint == service) {
            if server.ips.is_empty() {
                return HttpResponse::ServiceUnavailable().body("503 Service Unavailable");
            }
            let host = format!("http://{}:{}/{}", server.ips[0], server.port, path);
            let client = awc::Client::new();
            let mut req = client.request(method.clone(), host);
            for (key, value) in header.iter() {
                req = req.append_header((key.clone(), value.clone()));
            }
            let req = req.send_stream(payload)
                .await;
            let mut res = match req {
                Ok(res) => res,
                Err(_) => return HttpResponse::ServiceUnavailable().body("503 Reverse proxy service error"),
            };
            let mut response =  HttpResponse::build(res.status());
            for (key, value) in res.headers() {
                response.append_header((key.clone(), value.clone()));
            }
            return response.body(res.body().await.unwrap_or(Bytes::new()));
        }
    }
    let web_path = data.iter().filter(|x| x.endpoint.starts_with("path@")).map(|x|x.clone()).collect::<Vec<HttpServiceNode>>();
    {
        let urls = url.clone()
            .split("/")
            .map(|x|x.to_string())
            .collect::<Vec<_>>();
        let mut server:Option<HttpServiceNode> = None;
        let mut length = 0;
        for x in web_path.iter(){
            let endpoint = x.endpoint.split("@").collect::<Vec<&str>>()
                .last()
                .map(|x| x.to_string())
                .unwrap_or("".to_string());
            let endpoint_urls = endpoint.split("/").map(|x|x.to_string()).collect::<Vec<_>>();
            if endpoint_urls.len() > length {
                let mut flag = true;
                for (i, url) in urls.iter().enumerate() {
                    if i >= endpoint_urls.len() {
                        break;
                    }
                    if url != &endpoint_urls[i] {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    server = Some(x.clone());
                    length = endpoint_urls.len();
                }
            }
        }
        if let Some(x) = server {
            if x.ips.is_empty() {
                return HttpResponse::ServiceUnavailable().body("503 Service Unavailable");
            }
            let host = format!("http://{}:{}/{}", x.ips[0], x.port, url);
            let client = awc::Client::new();
            let mut req = client.request(method.clone(), host);
            for (key, value) in header.iter() {
                req = req.append_header((key.clone(), value.clone()));
            }
            let req = req.send_stream(payload)
                .await;
            let mut res = match req {
                Ok(res) => res,
                Err(_) => return HttpResponse::ServiceUnavailable().body("503 Reverse proxy service error"),
            };
            let mut response =  HttpResponse::build(res.status());
            for (key, value) in res.headers() {
                response.append_header((key.clone(), value.clone()));
            }
            return response.body(res.body().await.unwrap_or(Bytes::new()));
        }
    }
    HttpResponse::NotFound().body("404 notfound")
}


#[derive(Debug, Clone)]
pub enum ApiVersion {
    V1,
}
impl TryFrom<&str> for ApiVersion {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "v1" => Ok(ApiVersion::V1),
            _ => Err("404 notfound"),
        }
    }
}
