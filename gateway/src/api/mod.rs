use std::collections::HashSet;
use actix_session::Session;
use actix_web::http::header::REFERER;
use actix_web::web::{Bytes, Payload};
use actix_web::{web, HttpRequest, HttpResponse};
use lib_config::naming::HttpServiceNode;
use url::Url;
use awc::{Client, ClientResponse};
use lazy_static::lazy_static;

lazy_static! {
    static ref STATIC_EXTENSIONS: HashSet<&'static str> = [
        "css", "js", "png", "jpg", "jpeg", "ico", "svg", "ttf", "eot", "map",
        "html", "json", "xml", "txt", "md", "mdx"
    ].iter().cloned().collect();
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

async fn send_request(
    client: &Client,
    method: &str,
    host: String,
    headers: &actix_web::http::header::HeaderMap,
    payload: Payload,
) -> Result<HttpResponse, HttpResponse> {
    let mut req = client.request(method.parse().unwrap(), host);
    for (key, value) in headers.iter() {
        req = req.append_header((key.clone(), value.clone()));
    }
    let mut res = req.send_stream(payload)
        .await
        .map_err(|_| HttpResponse::ServiceUnavailable().body("503 Reverse proxy service error"))?;
    let body = res.body()
        .await
        .map_err(|_| HttpResponse::ServiceUnavailable().body("503 Reverse proxy service error"))?;
    
    Ok(build_response(res, body))
}

fn build_response<T>(res: ClientResponse<T>, body: Bytes) -> HttpResponse {
    let mut response = HttpResponse::build(res.status());
    for (key, value) in res.headers() {
        response.append_header((key.clone(), value.clone()));
    }
    response.body(body)
}

fn extract_path(url: &str) -> Vec<String> {
    Url::parse(url).map_or_else(
        |_| vec![],
        |parsed_url| parsed_url.path_segments()
            .map(|segments| segments.map(str::to_owned).collect())
            .unwrap_or_default(),
    )
}

fn is_static_resource(path: &str) -> bool {
    STATIC_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
}

fn process_urls(request: &HttpRequest, url: &str) -> Vec<String> {
    if let Some(referer_header) = request.headers().get(REFERER) {
        if let Ok(referer_str) = referer_header.to_str() {
            if let Ok(referer_url) = Url::parse(referer_str) {
                if is_static_resource(referer_url.path()) {
                    return extract_path(referer_url.path());
                }
            }
        }
    }
    extract_path(url)
}

pub async fn endpoint(request: HttpRequest, data: web::Data<Vec<HttpServiceNode>>, payload: Payload, _session: Session) -> HttpResponse {
    let url = request.uri().to_string();
    let method = request.method().as_str();
    let headers = request.headers().clone();
    let client = Client::default();

    if url.starts_with("/api/") {
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() < 4 {
            return HttpResponse::NotFound().body("404 notfound");
        }
        let service = parts[3];
        let path = parts[4..].join("/");
        if let Some(server) = data.iter().find(|x| x.endpoint == service) {
            if server.ips.is_empty() {
                return HttpResponse::ServiceUnavailable().body("503 Service Unavailable");
            }
            let host = format!("http://{}:{}/{}", server.ips[0], server.port, path);
            return send_request(&client, method, host, &headers, payload).await.unwrap_or_else(|e| e);
        }
    }

    let web_path: Vec<HttpServiceNode> = data.iter().filter(|x| x.endpoint.starts_with("path@")).cloned().collect();
    let urls = process_urls(&request, &url);
    let mut server: Option<HttpServiceNode> = None;
    let mut length = 0;

    for x in web_path.iter() {
        let endpoint = x.endpoint.split('@').last().unwrap_or("").to_string();
        let endpoint_urls = extract_path(&endpoint);
        if endpoint_urls.len() > length {
            let mut flag = true;
            for (i, url_part) in urls.iter().enumerate() {
                if i >= endpoint_urls.len() {
                    break;
                }
                if url_part != &endpoint_urls[i] {
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
        return send_request(&client, method, host, &headers, payload).await.unwrap_or_else(|e| e);
    }

    HttpResponse::NotFound().body("404 notfound")
}
