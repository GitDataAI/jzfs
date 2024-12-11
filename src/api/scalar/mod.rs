use crate::api::handler::repo::branchs::branch::__path_api_repo_branch;
use crate::api::handler::repo::branchs::rename::__path_api_repo_branch_rename;
use crate::api::handler::repo::branchs::protect::__path_api_repo_branch_protect;
use crate::api::handler::repo::branchs::new::__path_api_repo_branch_new;
use crate::api::handler::repo::branchs::merge::__path_api_repo_branch_merge;
use crate::api::handler::repo::branchs::del::__path_api_repo_branch_del;
use crate::api::handler::repo::branchs::conflicts::__path_api_repo_branch_check_merge;
use crate::api::handler::repo::info::__path_api_repo_info;
use crate::api::handler::group::team::__path_api_group_team_get;
use crate::api::handler::group::member::__path_api_group_member;
use crate::api::handler::group::repo::__path_api_group_repo_get;
use crate::api::handler::users::email::__path_api_user_email_unbind;
use crate::api::handler::users::email::__path_api_user_email_bind;
use crate::api::handler::repo::object::__path_api_repo_object_tree;
use crate::api::handler::owner::watch::__path_api_owner_watcher;
use crate::api::handler::users::star::__path_api_user_star_remove;
use crate::api::handler::users::star::__path_api_user_star_add;
use crate::api::handler::owner::star::__path_api_owner_star;
use crate::api::handler::users::avatar::__path_api_user_avatar_upload;
use crate::api::handler::users::avatar::__path_api_user_avatar_delete;
use crate::api::handler::owner::avatar::__path_api_owner_avatar;
use crate::api::handler::users::keys::__path_api_users_key_remove;
use crate::api::handler::users::keys::__path_api_users_key_create;
use crate::api::handler::owner::keys::__path_api_owner_keys;
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
        api_owner_keys,
        api_owner_avatar,
        api_owner_star,
        api_owner_watcher,
    
        api_user_apply,
        api_user_local,
        api_users_login_name,
        api_user_logout,
        api_user_reset_passwd_profile,
        api_user_setting,
        api_users_key_create,
        api_users_key_remove,
        api_user_avatar_upload,
        api_user_avatar_delete,
        api_user_star_remove,
        api_user_star_add,
        api_user_email_unbind,
        api_user_email_bind,
    
        api_email_rand_captcha,
        api_email_captcha_check,
        api_email_forget,


        api_group_create,
        api_group_info,
        api_group_repo_get,
        api_group_member,
        api_group_team_get,
    
    
        api_repo_create,
        api_repo_object_tree,
        api_repo_branch,
        api_repo_info,
        api_repo_branch_new,
        api_repo_branch_del,
        api_repo_branch_check_merge,
        api_repo_branch_merge,
        api_repo_branch_protect,
        api_repo_branch_rename,
    
        api_team_by_user,
        api_teams_create,
        api_team_info,
        api_team_group_invite,
        api_list_team,
    )
)]
pub struct ApiDoc;
