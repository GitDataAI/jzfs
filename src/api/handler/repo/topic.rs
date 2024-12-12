use actix_web::{web, Responder};
use uuid::Uuid;
use crate::api::dto::repo::RepoTopic;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "repos",
    path = "/api/v1/repo/{repo}/topic",
    params(
        ("repo" = Uuid, description = "Repo Uid"),
    ),
    responses(
        (status = 200, description = "Repo found successfully"),
        (status = 400, description = "Repo Not Found"),
    ),
)]
pub async fn api_repo_topic(
    service: web::Data<Service>,
    path: web::Path<Uuid>
)
-> impl Responder
{
    let repo_id = path.into_inner();
    match service.repo.topics(repo_id).await{
        Ok(x)=>{
            R::<Vec<String>>{
                code: 200,
                data: Option::from(x),
                msg: Option::from("[Ok]".to_string())
            }
        }
        Err(e)=>{
            R::<Vec<String>>{
                code: 400,
                data: Option::from(vec![]),
                msg: Option::from(format!("[Error]: {}",e.to_string()).to_string())
            }
        }
    }
}

#[utoipa::path(
    post,
    tag = "repos",
    path = "/api/v1/repo/{repo}/topic",
    params(
        ("repo" = Uuid, description = "Repo Uid"),
    ),
    request_body(content = RepoTopic, description = "Add Topic", content_type = "application/json"),
    responses(
        (status = 200, description = "Repo found successfully"),
        (status = 400, description = "Repo Not Found"),
    )
)]
pub async fn api_repo_topic_add(
    service: web::Data<Service>,
    path: web::Path<Uuid>,
    topic: web::Json<RepoTopic>
)
-> impl Responder
{
    let repo_id = path.into_inner();
    match service.repo.add_topic(repo_id, topic.topic.to_string()).await{
        Ok(_)=>{
            R::<bool>{
                code: 200,
                data: Option::from(true),
               msg: Option::from("[Ok]".to_string())
            }
        }
        Err(e)=>{
            R::<bool>{
                code: 400,
                data: Option::from(false),
                msg: Option::from(format!("[Error]: {}",e.to_string()).to_string())
            }
        }
    }
}

#[utoipa::path(
    delete,
    tag = "repos",
    path = "/api/v1/repo/{repo}/topic",
    params(
        ("repo" = Uuid, description = "Repo Uid"),
    ),
    request_body(content = RepoTopic, description = "Delete Topic", content_type = "application/json"),
    responses(
        (status = 200, description = "Repo found successfully"),
        (status = 400, description = "Repo Not Found"),
    )
)]
pub async fn api_repo_topic_del(
    service: web::Data<Service>,
    path: web::Path<Uuid>,
    topic: web::Json<RepoTopic>
)
-> impl Responder
{
    let repo_id = path.into_inner();
    match service.repo.del_topic(repo_id, topic.topic.to_string()).await{
        Ok(_)=>{
            R::<bool>{
                code: 200,
                data: Option::from(true),
               msg: Option::from("[Ok]".to_string())
            }
        }
        Err(e)=>{
            R::<bool>{
                code: 400,
                data: Option::from(false),
                msg: Option::from(format!("[Error]: {}",e.to_string()).to_string())
            }
        }
    }
}