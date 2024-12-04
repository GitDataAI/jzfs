use crate::api::dto::repo::RepoCreate;
use crate::api::service::Service;
use crate::utils::r::R;
use actix_session::Session;
use actix_web::{web, Responder};

#[utoipa::path(
    post,
    tag = "team",
    path = "/api/v1/repo/create",
    request_body = RepoCreate,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_repo_create(
    session: Session,
    dto: web::Json<RepoCreate>,
    service: web::Data<Service>,
) -> impl Responder
{
    let session = service.check.check_session(session).await;
    if !session.is_ok() {
        return R {
            code: 401,
            data: None,
            msg: Option::from("unauthorized".to_string()),
        };
    }
    let session = session.unwrap();
    match service
        .repo
        .create_repo(dto.into_inner(), session.uid)
        .await {
        Ok(_) => {
            R::<String>{
                code: 200,
                data: None,
                msg: Option::from("success".to_string()),
            }
        },
        Err(e) => {
            R {
                code: 500,
                data: None,
                msg: Option::from(e.to_string()),
            }
        }
    }
}