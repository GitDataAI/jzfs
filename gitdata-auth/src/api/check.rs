use actix_web::web;
use serde::Deserialize;
use lib_entity::write::AppWrite;
use crate::server::AppAuthState;


#[derive(Deserialize)]
pub struct CheckQuery {
    pub username: Option<String>,
    pub email: Option<String>,
}



/*
 * @Description: 检查用户名或邮箱是否存在
 * @Param: {CheckQuery} query
 * @Return: {AppWrite<bool>}
 * @Author: ZhenYi
 */
pub async fn auth_check(
    state: web::Data<AppAuthState>,
    query: web::Path<CheckQuery>
)
    -> impl actix_web::Responder
{
    let query = query.into_inner();
    if query.username.is_some() {
        let username = query.username.unwrap();
        return if state.check_have_username(username).await.unwrap_or_else(|_| false) {
            AppWrite::ok(true)
        } else {
            AppWrite::ok(false)
        }
            
    }
    if query.email.is_some() {
        let email = query.email.unwrap();
        return if state.check_have_email(email).await.unwrap_or_else(|_| false) {
            AppWrite::ok(true)
        } else {
            AppWrite::ok(false)
        }
    }
    AppWrite::ok(false)
}

