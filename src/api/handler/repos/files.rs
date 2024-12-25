use crate::api::dto::repo_dto::RepoFileUpload;
use crate::api::SERVER;
use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::handler::check_session;


pub async fn api_repo_file_upload(
    dto: web::Json<RepoFileUpload>,
    session: Session,
    service: SERVER,
    path: web::Path<(String,String,String)>,
)
-> impl Responder
{
    let (owner,repo,branch) = path.into_inner();
    let session_model = match check_session(session).await{
        Ok(session_model)=>session_model,
        Err(e)=>return AppWrite::<String>::unauthorized(e.to_string())
    };
    let repo_id = match service.repo_service().owner_name_by_model(owner,repo).await{
        Ok(repo_id)=>repo_id,
        Err(e)=>return AppWrite::not_found(e.to_string())
    };
    let access = match service.repo_service().repo_access_user(repo_id.uid,session_model.uid).await{
        Ok(access)=>access,
        Err(e)=>return AppWrite::forbidden(e.to_string())
    };
    if !vec![1,2,3].contains(&access){
        return AppWrite::forbidden("forbidden".to_string())
    }
    let main_email = match service.user_service().email(session_model.uid).await{
        Ok(email)=>email,
        Err(e)=>return AppWrite::forbidden(e.to_string())
    };
    let main_email = {
        let email = main_email
            .into_iter()
            .filter(|x| x.main == true)
            .collect::<Vec<_>>();
        if email.len() > 0 {
            email[0].email.clone()
        }else {
            "null@null.com".to_string()
        }
    };
    
    match service.repo_service().add_file(
       repo_id.uid,
       branch,
       dto.file_name.clone(),
       dto.path.clone(),
       dto.content.clone(),
       dto.msg.clone(),
       session_model.username,
       main_email
    ).await{
        Ok(_) => {
            AppWrite::success("success".to_string())
        }
        Err(_) => {
            AppWrite::internal_server_error("internal server error".to_string())
        }
    }
}