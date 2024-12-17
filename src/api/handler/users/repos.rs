use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::metadata::model::repo::repo::Model;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "users",
    path = "/api/v1/users/once/{user}/repos",
    params(
        ( "user",Path, description = "User Name"),
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_users_repos(
    service: web::Data<MetaService>,
    user: web::Path<String>
) -> impl Responder
{
    let user_id = match service.user_service().username_to_uid(user.into_inner()).await{
        Ok(x) => x,
        Err(e) => return AppWrite::<Vec<Model>>::fail(e.to_string())
    };
    let models = service.user_service().owner_repo(user_id).await;
    match models{
        Ok(models) => AppWrite::ok(models),
        Err(e) => AppWrite::fail(e.to_string())
    }
}