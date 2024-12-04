use crate::api::dto::common::ListOption;
use crate::api::dto::search::SearchTeamOptions;
use crate::api::service::Service;
use crate::utils::r::R;
use actix_web::{web, Responder};

#[utoipa::path(
    get,
    tag = "search",
    path = "/api/v1/search/team",
    request_body = ListOption<SearchTeamOptions>,
    responses(
        (status = 200, description = "Ok"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_list_team(
    service: web::Data<Service>,
    option: web::Json<ListOption<SearchTeamOptions>>
) -> impl Responder
{
    let result = service.team.list(option.into_inner()).await;
    R{
        code: 200,
        msg: Option::from("success".to_string()),
        data: Some(result),
    }
}