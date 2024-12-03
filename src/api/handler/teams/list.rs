use crate::api::dto::common::ListOption;
use crate::api::dto::search::SearchTeamOptions;
use crate::api::service::Service;
use crate::utils::r::R;
use actix_web::{web, Responder};

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