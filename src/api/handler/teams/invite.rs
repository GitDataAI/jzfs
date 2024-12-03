use actix_session::Session;
use actix_web::{web, Responder};
use uuid::Uuid;
use crate::api::dto::team::TeamInvite;
use crate::api::service::Service;
use crate::utils::r::R;

pub async fn api_team_group_invite(
    session: Session,
    path: web::Path<(Uuid,Uuid)>,
    dto: web::Json<TeamInvite>,
    service: web::Data<Service>
) -> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<String>{
            code: 401,
            msg: Option::from("unauthorized".to_string()),
            data: None
        }
    }
    let g = service.check.check_exits_group(path.0.clone()).await;
    if !g{
        return R{
            code: 400,
            msg: Option::from("group not exits".to_string()),
            data: None
        }
    }
    match service.team.invite_user(path.0.clone(), path.1.clone(), model.unwrap().uid, dto.0.email).await{
        Ok(_) => R{
            code: 200,
            msg: Option::from("success".to_string()),
            data: None
        },
        Err(e) => R{
            code: 500,
            msg: Option::from(e.to_string()),
            data: None
        }
    }
}
