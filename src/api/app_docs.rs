use crate::api::handler::email::captcha::__path_api_email_rand_captcha;
use crate::api::handler::email::captcha::__path_api_email_captcha_check;
use crate::api::handler::repos::info::__path_api_repo_info_get;
use crate::api::handler::users::starred::__path_api_users_starred;
use crate::api::handler::users::search::__path_api_users_search;
use crate::api::handler::repos::search::__path_api_repo_search;
use crate::api::handler::users::repos::__path_api_users_repos;
use crate::api::handler::users::follower::__path_api_users_following;
use crate::api::handler::users::follower::__path_api_users_followed;
use crate::api::handler::users::info::__path_api_users_info;
use crate::api::handler::groups::labels::__path_api_groups_labels;
use crate::api::handler::groups::labels::__path_api_groups_labels_update;
use crate::api::handler::groups::labels::__path_api_groups_labels_delete;
use crate::api::handler::groups::labels::__path_api_groups_labels_create;
use crate::api::handler::user::repos::__path_api_user_repo_create;
use crate::api::handler::groups::repos::__path_api_groups_repo_create;
use crate::api::handler::groups::repos::__path_api_groups_repo;
use crate::api::handler::user::subscriptions::__path_api_user_subscriptions;
use crate::api::handler::user::subscriptions::__path_api_user_subscription_remove;
use crate::api::handler::user::subscriptions::__path_api_user_subscription_add;
use crate::api::handler::groups::members::__path_api_user_groups;
use crate::api::handler::groups::members::__path_api_groups_members;
use crate::api::handler::groups::members::__path_api_groups_member_remove;
use crate::api::handler::groups::avatar::__path_api_groups_avatar_upload;
use crate::api::handler::groups::avatar::__path_api_groups_avatar;
use crate::api::handler::groups::members::__path_api_groups_member_add;
use crate::api::handler::groups::search::__path_api_groups_search;
use crate::api::handler::groups::info::__path_api_groups_info;
use crate::api::handler::groups::create::__path_api_groups_create;
use crate::api::handler::users::logout::__path_api_users_logout;
use crate::api::handler::users::login::__path_api_users_login_name;
use crate::api::handler::users::login::__path_api_users_login_email;
use crate::api::handler::users::apply::__path_api_users_apply;
use crate::api::handler::user::follower::__path_api_user_unfollow;
use crate::api::handler::user::starred::__path_api_user_staring;
use crate::api::handler::user::starred::__path_api_user_star_remove;
use crate::api::handler::user::starred::__path_api_user_star_add;
use crate::api::handler::user::setting::__path_api_user_setting_patch;
use crate::api::handler::user::setting::__path_api_user_setting_get;
use crate::api::handler::users::reset::__path_api_user_reset_passwd_profile;
use crate::api::handler::users::reset::__path_api_user_reset_passwd_forget;
use crate::api::handler::user::repos::__path_api_user_repo;
use crate::api::handler::user::keys::__path_api_user_keys;
use crate::api::handler::user::keys::__path_api_user_key_remove;
use crate::api::handler::user::keys::__path_api_user_key_create;
use crate::api::handler::user::follower::__path_api_user_follower;
use crate::api::handler::user::follower::__path_api_user_followed;
use crate::api::handler::user::follower::__path_api_user_follow;
use crate::api::handler::user::emails::__path_api_user_email;
use crate::api::handler::user::emails::__path_api_user_email_unbind;
use crate::api::handler::user::emails::__path_api_user_email_bind;
use crate::api::handler::user::avatar::__path_api_user_avatar;
use crate::api::handler::user::avatar::__path_api_user_avatar_upload;
use crate::api::handler::user::avatar::__path_api_user_avatar_delete;
use crate::api::handler::user::keys::__path_api_use_key_once;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(description = "GitDataAi Api description"),
    paths(
        api_users_apply,
        api_users_login_name,
        api_users_login_email,
        api_users_logout,
        api_user_reset_passwd_profile,
        api_user_reset_passwd_forget,
        api_users_info,
        api_users_following,
        api_users_followed,
        api_users_repos,
        api_users_search,
        api_users_starred,
    
        api_email_rand_captcha,
        api_email_captcha_check,
        

        api_user_avatar,
        api_user_avatar_upload,
        api_user_avatar_delete,
        api_user_email,
        api_user_email_bind,
        api_user_email_unbind,
        api_user_follower,
        api_user_followed,
        api_user_follow,
        api_user_unfollow,
        api_user_keys,
        api_use_key_once,
        api_user_key_create,
        api_user_key_remove,
        api_user_repo,
        api_user_repo_create,
        api_user_groups,

        api_user_setting_get,
        api_user_setting_patch,
        api_user_staring,
        api_user_star_add,
        api_user_star_remove,
        api_user_subscriptions,
        api_user_subscription_add,
        api_user_subscription_remove,
    
        api_groups_search,
        api_groups_create,
        api_groups_info,
        api_groups_members,
        api_groups_member_add,
        api_groups_member_remove,
        api_groups_avatar,
        api_groups_avatar_upload,
        api_groups_repo,
        api_groups_repo_create,
        api_groups_labels,
        api_groups_labels_create,
        api_groups_labels_delete,
        api_groups_labels_update,

        api_repo_search,
        api_repo_info_get,
    ),
)]
pub struct ApiDoc;
