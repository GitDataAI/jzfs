use crate::endpoint::Endpoint;
use actix_web::web::{Data, Json};
use actix_web::Responder;
use workhorse::schema::users::UserCheckParam;

pub async fn user_check(
    endpoint: Data<Endpoint>,
    param: Json<UserCheckParam>,
) -> impl Responder {
    endpoint.user_check(param.into_inner()).await
}