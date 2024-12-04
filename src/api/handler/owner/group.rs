use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::service::Service;
use crate::metadata::model::groups::group;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "owner",
    path = "/api/v1/owner/group",
    responses(
            (status = 200, description = "OK"),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 403, description = "Get Teams Failed"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_owner_group(
    session: Session,
    service: web::Data<Service>
)
-> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<Vec<group::Model>>{
            code: 402,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    let item = service.team.byuser(model.unwrap().uid)
        .await;
    if item.is_err(){
        return R::<Vec<group::Model>>{
            code: 403,
            msg: Option::from("[Error] Get Teams Failed".to_string()),
            data: None,
        }
    }
    let groups = item.unwrap()
        .iter()
        .map(|x| x.group_id)
        .collect::<Vec<_>>();
    match service.group.infos(groups).await{
        Ok(x) => {
            return R::<Vec<group::Model>>{
                code: 200,
                msg: Option::from("[Success] Get Teams Success".to_string()),
                data: Some(x),
            }
        },
        Err(e) => {
            return R::<Vec<group::Model>>{
                code: 405,
                msg: Option::from(format!("[Error] {}",e.to_string()).to_string()),
                data: None,
            }
        }
    }
    
}