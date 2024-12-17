use actix_web::http::header::HeaderValue;
use actix_web::http::{header, StatusCode};
use actix_web::web::Payload;
use actix_web::{web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use flate2::read::GzDecoder;
use futures_util::{StreamExt};
use std::collections::HashMap;
use std::io;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use actix_files::NamedFile;
use time::format_description;
use tracing::info;
use crate::metadata::service::MetaService;
use crate::ROOT_PATH;

pub async fn info_refs(req: HttpRequest, path: web::Path<(String, String)>, query: web::Query<HashMap<String, String>>,service: web::Data<MetaService>) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let service_name = query.get("service").unwrap_or(&"".to_string()).to_string();
    if service_name != "git-receive-pack" && service_name != "git-upload-pack" {
        return HttpResponse::BadRequest().body("Invalid service name");
    }
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let service_name = service_name.replace("git-", "");
    let version = req.headers().get("Git-Protocol").unwrap_or(&HeaderValue::from_str("").unwrap()).to_str().map(|s| s.to_string()).unwrap_or("".to_string());
    let mut cmd = Command::new("git");
    cmd.arg(service_name.clone());
    cmd.arg("--stateless-rpc");
    cmd.arg("--advertise-refs");
    cmd.arg(".");
    cmd.current_dir(PathBuf::from(ROOT_PATH).join(repo_path.clone()));
    if !version.is_empty() {
        cmd.env("GIT_PROTOCOL", version.clone());
    }

    let output = match cmd.output() {
        Ok(output) =>{
            info!("Command status: {:?}", output.status);
            output
        },
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return HttpResponse::InternalServerError().body("Error running command");
        }
    };
    let mut resp = HttpResponseBuilder::new(StatusCode::OK);
    resp.append_header(("Content-Type", format!("application/x-git-{}-advertisement", service_name)));
    resp.append_header(("Pragma","no-cache"));
    resp.append_header(("Cache-Control","no-cache, max-age=0, must-revalidate"));
    resp.append_header(("Expires","Fri, 01 Jan 1980 00:00:00 GMT"));

    let mut body = String::new();

    match service_name.as_str() {
        "upload-pack" => {
            body.push_str(&"001e# service=git-upload-pack\n".to_string());
            body.push_str("0000");
        },
        "receive-pack" => {
            body.push_str(&"001f# service=git-receive-pack\n".to_string());
            body.push_str("0000");
        },
        _ => {}
    }
    body.push_str(&String::from_utf8(output.stdout).unwrap());
    dbg!(&body);
    resp.body(body.as_bytes().to_vec())
}


pub async fn git_upload_pack(http_request: HttpRequest,path: web::Path<(String, String)>, mut payload: Payload, service: web::Data<MetaService>) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    
    
    let version = http_request.headers().get("Git-Protocol").unwrap_or(&HeaderValue::from_str("").unwrap()).to_str().map(|s| s.to_string()).unwrap_or("".to_string());

    let mut resp = HttpResponseBuilder::new(StatusCode::OK);
    resp.append_header(("Content-Type", "application/x-git-upload-pack-advertise"));
    resp.append_header(("Connection", "Keep-Alive"));
    resp.append_header(("Transfer-Encoding","chunked"));
    resp.append_header(("X-Content-Type-Options","nosniff"));
    let mut cmd = Command::new("git");
    cmd.arg("upload-pack");
    cmd.arg("--stateless-rpc");
    cmd.arg(".");
    if !version.is_empty() {
        cmd.env("GIT_PROTOCOL", version.clone());
    }
    cmd.stderr(Stdio::piped());
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.current_dir(PathBuf::from(ROOT_PATH).join(repo_path));
    
    
    let span = cmd.spawn();
    let mut span = match span {
        Ok(span) => span,
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return HttpResponse::InternalServerError().body("Error running command");
        }
    };
    
    let mut stdin = span.stdin.take().unwrap();
    let mut stdout = span.stdout.take().unwrap();
    let _stderr = span.stderr.take().unwrap();
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => bytes.extend_from_slice(&data),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to read request body: {}", e)),
        }
    }
    let body_data = match http_request.headers().get(header::CONTENT_ENCODING).and_then(|v| v.to_str().ok()) {
        Some("gzip") => {
            let mut decoder = GzDecoder::new(Cursor::new(bytes));
            let mut decoded_data = Vec::new();
            if let Err(e) = io::copy(&mut decoder, &mut decoded_data) {
                return HttpResponse::InternalServerError().body(format!("Failed to decode gzip body: {}", e));
            }
            decoded_data
        },
        _ => bytes.to_vec(),
    };
    if let Err(e) = stdin.write_all(&body_data) {
        return HttpResponse::InternalServerError().body(format!("Failed to write to git process: {}", e));
    }
    drop(stdin);

    let body_stream = actix_web::body::BodyStream::new(async_stream::stream! {
        let mut buffer = [0; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    yield Ok::<_, io::Error>(web::Bytes::copy_from_slice(&buffer[..n]))
                },
                Err(e) => {
                    eprintln!("Error reading stdout: {}", e);
                    break;
                }
            }
        }
    });
    resp.body(body_stream)
}

pub async fn git_receive_pack(
    http_request: HttpRequest,
    path: web::Path<(String, String)>,
    mut payload: Payload,
    service: web::Data<MetaService>,
) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let repo_full_path = PathBuf::from(ROOT_PATH).join(&repo_path);

    // 检查仓库是否是裸仓库
    if !repo_full_path.join("HEAD").exists() || !repo_full_path.join("config").exists() {
        return HttpResponse::BadRequest().body("Repository not found or invalid.");
    }

    // 确保仓库是裸仓库
    let is_bare_repo = match std::fs::read_to_string(repo_full_path.join("config")) {
        Ok(config) => config.contains("bare = true"),
        Err(_) => false,
    };
    if !is_bare_repo {
        return HttpResponse::BadRequest().body("Push operation requires a bare repository.");
    }

    // 处理 Git-Protocol 头信息
    let version = http_request
        .headers()
        .get("Git-Protocol")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let mut resp = HttpResponseBuilder::new(StatusCode::OK);
    resp.append_header(("Content-Type", "application/x-git-receive-pack-advertise"));
    resp.append_header(("Connection", "Keep-Alive"));
    resp.append_header(("Transfer-Encoding", "chunked"));
    resp.append_header(("X-Content-Type-Options", "nosniff"));

    // 启动 Git 命令
    let mut cmd = Command::new("git");
    cmd.arg("receive-pack");
    cmd.arg("--stateless-rpc");
    cmd.arg(".");
    if !version.is_empty() {
        cmd.env("GIT_PROTOCOL", version);
    }
    cmd.stderr(Stdio::piped());
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.current_dir(&repo_full_path);

    let mut git_process = match cmd.spawn() {
        Ok(process) => process,
        Err(e) => {
            info!("Error running git command: {}", e);
            return HttpResponse::InternalServerError().body("Error running git command");
        }
    };

    let mut stdin = git_process.stdin.take().unwrap();
    let mut stdout = git_process.stdout.take().unwrap();

    // 读取 Payload 数据
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => bytes.extend_from_slice(&data),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to read request body: {}", e)),
        }
    }

    // 处理压缩数据（gzip）
    let body_data = match http_request
        .headers()
        .get(header::CONTENT_ENCODING)
        .and_then(|v| v.to_str().ok())
    {
        Some(encoding) if encoding.contains("gzip") => {
            let mut decoder = GzDecoder::new(Cursor::new(bytes));
            let mut decoded_data = Vec::new();
            if let Err(e) = io::copy(&mut decoder, &mut decoded_data) {
                return HttpResponse::InternalServerError().body(format!("Failed to decode gzip body: {}", e));
            }
            decoded_data
        }
        _ => bytes.to_vec(),
    };

    // 写入到 Git 进程的 stdin
    if let Err(e) = stdin.write_all(&body_data) {
        return HttpResponse::InternalServerError().body(format!("Failed to write to git process: {}", e));
    }
    drop(stdin); // 关闭 stdin，通知 Git 命令已完成输入

    // 从 Git stdout 中读取数据并返回
    let body_stream = actix_web::body::BodyStream::new(async_stream::stream! {
        let mut buffer = [0; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => yield Ok::<_, io::Error>(web::Bytes::copy_from_slice(&buffer[..n])),
                Err(e) => {
                    eprintln!("Error reading stdout: {}", e);
                    break;
                }
            }
        }
    });

    resp.body(body_stream)
}

pub async fn get_text_file(http_request: HttpRequest, path: web::Path<(String, String)>, service: web::Data<MetaService>) -> impl Responder{
    let (owner, repo_name) = path.into_inner();
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let path = http_request.uri().to_string().replace(&repo_path.clone(), "");

    let mut resp = HashMap::new();
    resp.insert("Pragma".to_string(),"no-cache".to_string());
    resp.insert("Cache-Control".to_string(),"no-cache, max-age=0, must-revalidate".to_string());
    resp.insert("Expires".to_string(),"Fri, 01 Jan 1980 00:00:00 GMT".to_string());
    
    let req_file = PathBuf::from(ROOT_PATH).join(repo_path).join(path);
    if !req_file.exists() {
        return HttpResponse::NotFound().body("File not found");
    }
    match NamedFile::open(req_file) {
        Ok(mut named_file) => {
            named_file = named_file.use_last_modified(true);
            let mut response = named_file.into_response(&http_request);
            for (k, v) in resp.iter() {
                response.headers_mut().insert(
                    k.to_string().parse().unwrap(),
                    header::HeaderValue::from_str(v).unwrap(),
                );
            }

            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str("text/plain").unwrap(),
            );

            response
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to open file"),
    }
}

pub async fn objects_info_packs(http_request: HttpRequest,path: web::Path<(String, String)>, service: web::Data<MetaService>) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let path = "objects/info/packs".to_string();
    let mut map = HashMap::new();
    let time = time::OffsetDateTime::now_utc();
    let expires = time::OffsetDateTime::now_utc() + time::Duration::days(1);
    map.insert("Date".to_string(), time.format(&format_description::parse("%a, %d %b %Y %H:%M:%S GMT").unwrap()).unwrap());
    map.insert("Expires".to_string(), expires.format(&format_description::parse("%a, %d %b %Y %H:%M:%S GMT").unwrap()).unwrap());
    map.insert("Cache-Control".to_string(), "public, max-age=86400".to_string());
    let req_file = PathBuf::from(ROOT_PATH).join(repo_path).join(path);
    if !req_file.exists() {
        return HttpResponse::NotFound().body("File not found");
    }
    match NamedFile::open(req_file) {
        Ok(mut named_file) => {
            named_file = named_file.use_last_modified(true);
            let mut response = named_file.into_response(&http_request);
            for (k, v) in map.iter() {
                response.headers_mut().insert(
                    k.to_string().parse().unwrap(),
                    HeaderValue::from_str(v).unwrap(),
                );
            }

            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str("application/x-git-loose-object").unwrap(),
            );

            response
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to open file"),
    }
}


pub async fn objects_pack(http_request: HttpRequest, path: web::Path<(String, String, String)>, service: web::Data<MetaService>) -> impl Responder {
    let (owner, repo_name,pack_hash) = path.into_inner();
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let path = format!("objects/pack/{}", pack_hash);
    let url = http_request.uri().to_string().replace(&repo_path.clone(), "");

    let mut map = HashMap::new();
    let time = time::OffsetDateTime::now_utc();
    let expires = time::OffsetDateTime::now_utc() + time::Duration::days(1);
    map.insert("Date".to_string(), time.format(&format_description::parse("%a, %d %b %Y %H:%M:%S GMT").unwrap()).unwrap());
    map.insert("Expires".to_string(), expires.format(&format_description::parse("%a, %d %b %Y %H:%M:%S GMT").unwrap()).unwrap());
    map.insert("Cache-Control".to_string(), "public, max-age=86400".to_string());
    let mut xtype = "application/x-git-loose-object".to_string();
    if url.ends_with(".pack") {
        xtype = "application/x-git-packed-objects".to_string();
    } else if url.ends_with(".idx") {
        xtype = "application/x-git-packed-objects-toc".to_string();
    } else {
        xtype = "application/x-git-loose-object".to_string();
    }
    
    let req_file = PathBuf::from(ROOT_PATH).join(repo_path).join(path);
    if !req_file.exists() {
        return HttpResponse::NotFound().body("File not found");
    }
    match NamedFile::open(req_file) {
        Ok(mut named_file) => {
            named_file = named_file.use_last_modified(true);
            let mut response = named_file.into_response(&http_request);
            for (k, v) in map.iter() {
                response.headers_mut().insert(
                    k.to_string().parse().unwrap(),
                    HeaderValue::from_str(v).unwrap(),
                );
            }

            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str(&xtype).unwrap(),
            );

            response
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to open file"),
    }
    
}
