use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::service::Service;
use crate::metadata::model::repos::repo;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "repo",
    path = "/api/v1/owner/repo",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_owner_repo(
    session: Session,
    service: web::Data<Service>,
) -> impl Responder {
    let session = service.check.check_session(session).await;
    if !session.is_ok() {
        return R{
            code: 401,
            data: None,
            msg: Option::from("unauthorized".to_string()),
        }
    }
    let session = session.unwrap();
    match service.repo.owner(session.uid).await {
        Ok(x) => {
            R::<Vec<_>>{
                code: 200,
                data: Option::from(x),
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
