use crate::api::handler::users::setting::__path_api_user_setting;
use crate::api::handler::owner::setting::__path_api_owner_setting;
use crate::api::handler::owner::team::__path_api_owner_team;
use crate::api::handler::owner::repo::__path_api_owner_repo;
use crate::api::handler::owner::followers::__path_api_owner_follower;
use crate::api::handler::owner::email::__path_api_owner_email;
use crate::api::handler::repo::create::__path_api_repo_create;
use crate::api::handler::owner::group::__path_api_owner_group;
use crate::api::handler::teams::list::__path_api_list_team;
use crate::api::handler::email::forget::__path_api_email_forget;
use crate::api::handler::email::captcha::__path_api_email_captcha_check;
use crate::api::handler::users::login::__path_api_users_login_name;
use crate::api::handler::users::reset::__path_api_user_reset_passwd_profile;
use crate::api::handler::users::logout::__path_api_user_logout;
use crate::api::handler::users::localdata::__path_api_user_local;
use crate::api::handler::users::apply::__path_api_user_apply;
use crate::api::handler::email::captcha::__path_api_email_rand_captcha;
use crate::api::handler::teams::info::__path_api_team_info;
use crate::api::handler::teams::byuser::__path_api_team_by_user;
use crate::api::handler::teams::create::__path_api_teams_create;
use crate::api::handler::teams::invite::__path_api_team_group_invite;
use crate::api::handler::group::creat::__path_api_group_create;
use crate::api::handler::group::info::__path_api_group_info;

use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

#[derive(OpenApi)]
#[openapi(
    info(description = "GitDataAi Api description"),
    paths(
        api_owner_email,
        api_owner_follower,
        api_owner_group,
        api_owner_repo,
        api_owner_team,
        api_owner_setting,
    
        api_user_apply,
        api_user_local,
        api_users_login_name,
        api_user_logout,
        api_user_reset_passwd_profile,
        api_user_setting,
    

        api_email_rand_captcha,
        api_email_captcha_check,
        api_email_forget,


        api_group_create,
        api_group_info,

        api_repo_create,

        api_team_by_user,
        api_teams_create,
        api_team_info,
        api_team_group_invite,
        api_list_team,
    )
)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn init() -> Scalar<utoipa::openapi::OpenApi> {
        let scalar = Scalar::with_url("/scalar", ApiDoc::openapi());
        scalar
    }
}